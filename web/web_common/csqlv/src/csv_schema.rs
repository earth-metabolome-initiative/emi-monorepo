//! Submodule providing the CSV Schema struct, which loads a CSV directory and
//! processes it into a complete SQL database schema.
use indicatif::ProgressIterator;

use crate::metadata::CSVTableMetadata;

use crate::errors::CSVSchemaError;

/// Struct representing a CSV schema.
pub struct CSVSchema {
    tables: Vec<CSVTableMetadata>,
}

impl CSVSchema {
    /// Returns the number of tables in the schema.
    pub fn number_of_tables(&self) -> usize {
        self.tables.len()
    }
}

#[derive(Debug, Clone, Default)]
/// Builder for the CSV schema.
pub struct CSVSchemaBuilder {
    include_gz: bool,
}

impl CSVSchemaBuilder {
    /// Create a new CSV schema builder.
    pub fn new() -> Self {
        Self { include_gz: false }
    }

    /// Include .gz files in the schema.
    pub fn include_gz(&mut self) -> &mut Self {
        self.include_gz = true;
        self
    }

    /// Create a new CSV schema from a directory of CSV files.
    pub fn from_dir(self, dir: &str) -> Result<CSVSchema, CSVSchemaError> {
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
        let tables = paths
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
            .map(CSVTableMetadata::from_csv)
            .collect::<Result<Vec<_>, CSVSchemaError>>()?;

        // We check that the tables have unique names.
        let unique_names = tables
            .iter()
            .map(|table| table.name.clone())
            .collect::<std::collections::HashSet<_>>();

        if unique_names.len() != tables.len() {
            return Err(CSVSchemaError::DuplicateTable(
                "Duplicate table name".to_string(),
            ));
        }

        // We check that all of the foreign tables are valid and that the foreign
        // columns actually exist in the foreign tables.
        tables
            .iter()
            .map(|table| table.validate_schema_compatibility(&tables))
            .collect::<Result<(), _>>()?;

        Ok(CSVSchema { tables })
    }
}
