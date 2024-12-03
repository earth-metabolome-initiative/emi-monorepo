//! Submodule providing the CSV Schema struct, which loads a CSV directory and
//! processes it into a complete SQL database schema.
use indicatif::ProgressIterator;

use crate::csv_table::CSVTable;
use crate::errors::CSVSchemaError;
use crate::metadata::CSVTableMetadata;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a CSV schema.
pub struct CSVSchema {
    table_metadatas: Vec<CSVTableMetadata>,
}

impl CSVSchema {
    /// Returns the number of tables in the schema.
    pub fn number_of_tables(&self) -> usize {
        self.table_metadatas.len()
    }

    /// Returns the tables in the schema.
    pub fn tables(&self) -> Vec<CSVTable<'_>> {
        self.table_metadatas
            .iter()
            .map(|table_metadata| CSVTable {
                schema: self,
                table_metadata,
            })
            .collect()
    }

    /// Returns a table from the provided table name.
    pub fn table_from_name(&self, table_name: &str) -> Result<CSVTable<'_>, CSVSchemaError> {
        let table_metadata = self
            .table_metadatas
            .iter()
            .find(|table| table.name == table_name)
            .ok_or(CSVSchemaError::InvalidTableName(table_name.to_string()))?;
        Ok(CSVTable {
            schema: self,
            table_metadata,
        })
    }

    /// Returns the SQL to generate the schema in PostgreSQL.
    pub fn to_postgres(&self) -> Result<String, CSVSchemaError> {
        let mut sql = String::new();
        for table in self.tables() {
            sql.push_str(&table.to_postgres());
            sql.push('\n');
            sql.push_str(&table.populate()?);
            sql.push('\n');
        }
        Ok(sql)
    }
}

#[derive(Debug, Clone, Default)]
/// Builder for the CSV schema.
pub struct CSVSchemaBuilder {
    include_gz: bool,
    container_directory: Option<String>,
}

impl CSVSchemaBuilder {
    /// Create a new CSV schema builder.
    pub fn new() -> Self {
        Self {
            include_gz: false,
            container_directory: None,
        }
    }

    /// Include .gz files in the schema.
    pub fn include_gz(mut self) -> Self {
        self.include_gz = true;
        self
    }

    /// Set the container directory.
    pub fn container_directory<S: ToString>(mut self, container_directory: S) -> Self {
        self.container_directory = Some(container_directory.to_string());
        self
    }

    /// Create a new CSV schema from a directory of CSV files.
    pub fn from_dir(self, dir: &str) -> Result<CSVSchema, CSVSchemaError> {
        // If the container directory is set, we need to prepend it to the directory.
        let container_directory = if let Some(container_directory) = self.container_directory {
            container_directory
        } else {
            dir.to_owned()
        };

        // We iterate across the files in the directory and create a list
        // of the documents we want to parse.
        let paths = std::fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let progress_bar = indicatif::ProgressBar::new(paths.len() as u64);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        // Next, we iterate in parallel over the list of files to process
        // each file in a separate thread.
        let table_metadatas = paths
            .iter()
            .progress_with(progress_bar)
            .filter_map(|path| {
                let path = path.to_str().unwrap();
                if path.ends_with(".csv") || path.ends_with(".csv.gz") && self.include_gz {
                    Some(path)
                } else {
                    None
                }
            })
            .map(|path| CSVTableMetadata::from_csv(&dir, &path, &container_directory))
            .collect::<Result<Vec<_>, CSVSchemaError>>()?;

        // We check that the tables have unique names.
        let unique_names = table_metadatas
            .iter()
            .map(|table| table.name.clone())
            .collect::<std::collections::HashSet<_>>();

        if unique_names.len() != table_metadatas.len() {
            return Err(CSVSchemaError::DuplicateTable(
                "Duplicate table name".to_string(),
            ));
        }

        // We check that all of the foreign tables are valid and that the foreign
        // columns actually exist in the foreign tables.
        table_metadatas
            .iter()
            .map(|table| table.validate_schema_compatibility(&table_metadatas))
            .collect::<Result<(), _>>()?;

        // We check that there are no loops in the schema caused by foreign keys.
        for original_table in &table_metadatas {
            let mut visited: std::collections::HashSet<&CSVTableMetadata> =
                std::collections::HashSet::new();
            let mut stack: Vec<&CSVTableMetadata> = vec![&original_table];

            while let Some(table) = stack.pop() {
                if visited.contains(&table) {
                    continue;
                }

                visited.insert(table);

                for column in &table.columns {
                    if let Some(foreign_table_name) = &column.foreign_table_name {
                        let foreign_table = table_metadatas
                            .iter()
                            .find(|table| table.name == *foreign_table_name)
                            .unwrap();

                        if original_table == foreign_table {
                            return Err(CSVSchemaError::Loop(original_table.name.clone()));
                        }

                        stack.push(foreign_table);
                    }
                }
            }
        }

        Ok(CSVSchema { table_metadatas })
    }
}
