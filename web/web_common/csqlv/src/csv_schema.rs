//! Submodule providing the CSV Schema struct, which loads a CSV directory and
//! processes it into a complete SQL database schema.
use std::path::Path;

use indicatif::ProgressIterator;

use crate::{
    csv_table::CSVTable,
    errors::CSVSchemaError,
    extensions::{has_compression_extension, has_supported_extension},
    metadata::CSVTableMetadata,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a CSV schema.
pub struct CSVSchema {
    table_metadatas: Vec<CSVTableMetadata>,
}

impl CSVSchema {
    #[must_use]
    /// Returns the number of tables in the schema.
    pub fn number_of_tables(&self) -> usize {
        self.table_metadatas.len()
    }

    #[must_use]
    /// Returns the tables in the schema.
    pub fn tables(&self) -> Vec<CSVTable<'_>> {
        self.table_metadatas
            .iter()
            .map(|table_metadata| CSVTable { schema: self, table_metadata })
            .collect()
    }

    #[must_use]
    /// Returns the tables in the schema alongside their priority score, sorted
    /// by descending priority.
    ///
    /// # Implementative details
    /// The priority score is determined by the score of the dependant tables + 1.
    ///
    pub fn tables_with_priority(&self) -> Vec<(CSVTable<'_>, usize)> {
        let mut table_priority: std::collections::HashMap<CSVTable<'_>, usize> =
            std::collections::HashMap::new();
        let mut changed = true;

        while changed {
            changed = false;
            for table in self.tables() {
                let priority = table
                    .dependant_tables()
                    .iter()
                    .filter(|dependant_table| dependant_table != &&table)
                    .map(|dependant_table| table_priority.get(dependant_table).unwrap_or(&0))
                    .max()
                    .copied()
                    .map_or(0, |x| x + 1);
                if let Some(previous_priority) = table_priority.insert(table, priority) {
                    if previous_priority != priority {
                        changed = true;
                    }
                } else {
                    changed = true;
                }
            }
        }

        let mut table_priority: Vec<_> = table_priority.into_iter().collect();

        table_priority.sort_by_key(|(table, priority)| (*priority, table.name().to_owned()));

        table_priority.reverse();

        table_priority
    }

    /// Returns a table from the provided table name.
    ///
    /// # Arguments
    /// * `table_name` - The name of the table to retrieve.
    ///
    /// # Errors
    /// * If the provided table name does not exist in the schema.
    pub fn table_from_name(&self, table_name: &str) -> Result<CSVTable<'_>, CSVSchemaError> {
        let table_metadata = self
            .table_metadatas
            .iter()
            .find(|table| table.name == table_name)
            .ok_or(CSVSchemaError::InvalidTableName(table_name.to_string()))?;
        Ok(CSVTable { schema: self, table_metadata })
    }

    #[must_use]
    /// Returns the SQL to generate the schema in `PostgreSQL`.
    pub fn to_sql(&self) -> Result<String, CSVSchemaError> {
        let mut sql = String::new();
        for table in self.tables_with_priority().iter().map(|(table, _)| table) {
            sql.push_str(&table.to_sql()?);
            sql.push('\n');
            sql.push_str(&table.populate()?);
            sql.push('\n');
        }
        Ok(sql)
    }

    #[must_use]
    /// Connectes to the provided [`Connection`](diesel::Connection) and executes the SQL to
    /// generate the schema.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect to the database.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    ///
    pub fn connect_and_create<C: diesel::Connection>(
        &self,
        url: &str,
    ) -> Result<(), CSVSchemaError> {
        let mut attempts = 0;
        loop {
            match C::establish(url) {
                Err(err) => {
                    if attempts >= 10 {
                        return Err(err.into());
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    attempts += 1;
                }
                Ok(mut conn) => return self.create(&mut conn),
            }
        }
    }

    #[must_use]
    /// Executes the SQL to generate the schema in `PostgreSQL`.
    pub fn create<C: diesel::Connection>(&self, conn: &mut C) -> Result<(), CSVSchemaError> {
        let sql = self.to_sql()?;
        Ok(conn.batch_execute(&sql)?)
    }

    #[must_use]
    /// Returns the SQL to delete all tables in the schema in `PostgreSQL`.
    ///
    /// # Implementative details
    /// The deletion happens following the reverse order of the foreign keys.
    pub fn to_sql_delete(&self) -> String {
        let mut sql = String::new();
        for table in self.tables_with_priority().into_iter().rev().map(|(table, _)| table) {
            sql.push_str(&table.to_sql_delete());
            sql.push('\n');
        }
        sql
    }

    #[must_use]
    /// Connectes to the provided [`Connection`](diesel::Connection) and executes the SQL to
    /// delete the schema.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect to the database.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    ///
    pub fn connect_and_delete<C: diesel::Connection>(
        &self,
        url: &str,
    ) -> Result<(), CSVSchemaError> {
        let mut attempts = 0;
        loop {
            match C::establish(url) {
                Err(err) => {
                    if attempts >= 10 {
                        return Err(err.into());
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    attempts += 1;
                }
                Ok(mut conn) => return self.delete(&mut conn),
            }
        }
    }

    #[must_use]
    /// Executes the SQL to delete the schema in `PostgreSQL`.
    pub fn delete<C: diesel::Connection>(&self, conn: &mut C) -> Result<(), CSVSchemaError> {
        let sql = self.to_sql_delete();
        Ok(conn.batch_execute(&sql)?)
    }
}

#[derive(Debug, Clone, Default)]
/// Builder for the CSV schema.
pub struct CSVSchemaBuilder {
    include_gz: bool,
    container_directory: Option<String>,
    singularize: bool,
    verbose: bool,
}

impl CSVSchemaBuilder {
    #[must_use]
    /// Create a new CSV schema builder.
    pub fn new() -> Self {
        Self { include_gz: false, container_directory: None, singularize: false, verbose: false }
    }

    #[must_use]
    /// Include .gz files in the schema.
    pub fn include_gz(mut self) -> Self {
        self.include_gz = true;
        self
    }

    #[must_use]
    /// Set the container directory.
    pub fn container_directory(mut self, container_directory: &str) -> Self {
        self.container_directory = Some(container_directory.to_owned());
        self
    }

    #[must_use]
    /// Set the verbosity of the schema builder.
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    #[must_use]
    /// Singularize the table names.
    pub fn singularize(mut self) -> Self {
        self.singularize = true;
        self
    }

    /// Create a new CSV schema from a directory of CSV files.
    ///
    /// # Arguments
    /// * `dir` - The directory containing the CSV files.
    ///
    /// # Errors
    /// * If the directory cannot be read.
    /// * If the schema contains duplicate tables.
    /// * If the schema contains loops in the foreign keys.
    ///
    /// # Panics
    /// * If the schema contains foreign keys that do not exist.
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

        let progress_bar = if self.verbose {
            let progress_bar = indicatif::ProgressBar::new(paths.len() as u64);
            progress_bar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
                    .unwrap()
                    .progress_chars("##-"),
            );
            progress_bar
        } else {
            indicatif::ProgressBar::hidden()
        };

        // Next, we iterate in parallel over the list of files to process
        // each file in a separate thread.
        let table_metadatas = paths
            .iter()
            .progress_with(progress_bar)
            .filter(|path| {
                let path: &Path = path.as_ref();
                has_supported_extension(path)
                    && (!has_compression_extension(path) || self.include_gz)
            })
            .map(|path| {
                CSVTableMetadata::from_csv(
                    dir,
                    path.as_ref(),
                    &container_directory,
                    self.singularize,
                )
            })
            .collect::<Result<Vec<_>, CSVSchemaError>>()?;

        // We check that the tables have unique names.
        let unique_names = table_metadatas
            .iter()
            .map(|table| table.name.clone())
            .collect::<std::collections::HashSet<_>>();

        if unique_names.len() != table_metadatas.len() {
            return Err(CSVSchemaError::DuplicateTable("Duplicate table name".to_string()));
        }

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

        let schema = CSVSchema { table_metadatas };

        // We check that all of the foreign tables are valid and that the foreign
        // columns actually exist in the foreign tables.
        schema
            .table_metadatas
            .iter()
            .try_for_each(|table| table.validate_schema_compatibility(&schema))?;

        Ok(schema)
    }
}
