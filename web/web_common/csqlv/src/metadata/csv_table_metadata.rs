//! Submodule providing the CSV Table struct and associated functions.
use std::path::Path;

use csv::Reader;
use inflector::Inflector;
use serde::{Deserialize, Serialize};

use super::csv_column_metadata::{CSVColumnMetadata, CSVColumnMetadataBuilder};
use crate::{
    CSVSchema,
    data_types::DataType,
    errors::CSVSchemaError,
    extensions::{delimiter_from_path, file_name_without_extension, has_compression_extension},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Struct representing a CSV table.
pub struct CSVTableMetadata {
    /// Name of the table.
    pub(crate) name: String,
    /// Path of the CSV file.
    pub(crate) path: String,
    /// Whether the table name should be singularized.
    pub(crate) singularize: bool,
    /// Number of rows in the table.
    pub(crate) number_of_rows: u64,
    /// Columns of the table.
    pub(crate) columns: Vec<CSVColumnMetadata>,
}

impl CSVTableMetadata {
    /// Returns the delimiter of the xSV file.
    ///
    /// # Panics
    /// * If the delimiter cannot be determined.
    pub fn delimiter(&self) -> char {
        delimiter_from_path(Path::new(&self.path)).unwrap()
    }

    /// Returns the name of the table to use as foreign key.
    pub fn foreign_table_name(&self) -> String {
        if self.singularize {
            Inflector::default().singularize(&self.name)
        } else {
            self.name.clone()
        }
    }

    /// Create a new `CSVTableMetadata`.
    fn parse_rows<R>(
        mut rows: Reader<R>,
        name: &str,
        path: String,
        singularize: bool,
    ) -> Result<Self, CSVSchemaError>
    where
        R: std::io::Read,
    {
        let mut column_builders = rows
            .headers()?
            .into_iter()
            .map(CSVColumnMetadataBuilder::new)
            .collect::<Result<Vec<_>, _>>()?;

        let mut number_of_rows = 0;
        for row in rows.records() {
            number_of_rows += 1;
            let row = row?;
            for (col, value) in column_builders.iter_mut().zip(row.iter()) {
                col.update(value);
            }
        }

        let mut columns = column_builders
            .into_iter()
            .map(CSVColumnMetadata::try_from)
            .map(|col| {
                col.map_err(|err| match err {
                    CSVSchemaError::NonUniquePrimaryKey { column_name, .. } => {
                        CSVSchemaError::NonUniquePrimaryKey {
                            column_name,
                            table_name: Some(name.to_owned()),
                        }
                    }
                    _ => err,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Check that there are no duplicate column names
        let mut unique_columns = std::collections::HashSet::new();
        for column in &columns {
            if !unique_columns.insert(column) {
                return Err(CSVSchemaError::DuplicateColumn {
                    column_name: column.name.clone(),
                    table_name: name.to_string(),
                });
            }
        }

        // If there is no column identified as the primary key, we add a new
        // column with the name "id".
        if !columns.iter().any(|col: &CSVColumnMetadata| col.primary_key) {
            columns.push(CSVColumnMetadata::new_primary_key(
                if number_of_rows < i16::MAX as u64 {
                    DataType::SmallSerial
                } else if number_of_rows < i32::MAX as u64 {
                    DataType::Serial
                } else {
                    DataType::BigSerial
                },
            )?);
        }

        Ok(Self { name: name.to_string(), path, number_of_rows, singularize, columns })
    }

    /// Create a new `CSVTableMetadata` from a CSV file.
    pub fn from_csv(
        root: &Path,
        path: &Path,
        docker_root: &Path,
        singularize: bool,
    ) -> Result<Self, CSVSchemaError> {
        // We determine the internal path of the file, by replacing the root
        // portion of the path with the docker root
        let docker_path = path
            .strip_prefix(root)
            .map(|p| docker_root.join(p))
            .map_err(|_| CSVSchemaError::InvalidPath(path.to_string_lossy().to_string()))?
            .to_string_lossy()
            .to_string();

        // We check that the provided path ends with .csv or .csv.gz
        let (Some(table_name), Some(delimiter)) =
            (file_name_without_extension(path), delimiter_from_path(path))
        else {
            return Err(CSVSchemaError::InvalidPath(path.to_string_lossy().to_string()));
        };

        // We check that there is no invalid character in the table name
        if table_name.contains(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            return Err(CSVSchemaError::InvalidTableName(table_name.to_string()));
        }

        let mut reader_builder = csv::ReaderBuilder::new();
        reader_builder.has_headers(true);
        reader_builder.trim(csv::Trim::All);
        reader_builder.flexible(true);
        reader_builder.delimiter(delimiter.try_into().unwrap());
        let file = std::fs::File::open(path)?;

        // We open the file CSV
        if has_compression_extension(path) {
            let decoder = flate2::read::GzDecoder::new(file);
            let reader = reader_builder.from_reader(decoder);
            Self::parse_rows(reader, table_name, docker_path, singularize)
        } else {
            let reader = reader_builder.from_reader(file);
            Self::parse_rows(reader, table_name, docker_path, singularize)
        }
    }

    pub(crate) fn gzip(&self) -> bool {
        has_compression_extension(Path::new(&self.path))
    }

    pub(crate) fn temporary_table_name(&self) -> String {
        format!("{}_temp", self.name)
    }

    pub(crate) fn validate_schema_compatibility(
        &self,
        schema: &CSVSchema,
    ) -> Result<(), CSVSchemaError> {
        self.columns.iter().filter(|column| column.foreign_table_name.is_some()).try_for_each(
            |column| {
                if let (Some(foreign_table_name), Some(foreign_column_name)) =
                    (&column.foreign_table_name, &column.foreign_column_name)
                {
                    let Ok(foreign_table) = schema.table_from_name(foreign_table_name) else {
                        return Err(CSVSchemaError::UnknownForeignKey {
                            table_name: self.name.clone(),
                            column_name: column.name(schema)?,
                            foreign_table_name: foreign_table_name.clone(),
                            foreign_column_name: foreign_column_name.clone(),
                        });
                    };

                    if !foreign_table.has_column(foreign_column_name) {
                        return Err(CSVSchemaError::UnknownForeignKey {
                            table_name: self.name.clone(),
                            column_name: column.name(schema)?,
                            foreign_table_name: foreign_table_name.clone(),
                            foreign_column_name: foreign_column_name.clone(),
                        });
                    }
                }
                Ok(())
            },
        )
    }
}
