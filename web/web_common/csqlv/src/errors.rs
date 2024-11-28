//! Submodule providing errors that may occur during CSV schema processing.

use std::error::Error;

#[derive(Debug)]
/// Enum representing errors that may occur during CSV schema processing.
pub enum CSVSchemaError {
    InvalidPath(String),
    InvalidTableName(String),
    InvalidColumnName(String),
    DuplicateColumn(String),
    DuplicateTable(String),
    CSVError(csv::Error),
    IOError(std::io::Error),
    UnknownDataType(String),
    UnknownForeignKey {
        table_name: String,
        column_name: String,
        foreign_table_name: String,
        foreign_column_name: String,
    },
    InvalidTemporaryTableName(String),
    EmptyColumn,
}

impl From<csv::Error> for CSVSchemaError {
    fn from(err: csv::Error) -> CSVSchemaError {
        CSVSchemaError::CSVError(err)
    }
}

impl From<std::io::Error> for CSVSchemaError {
    fn from(err: std::io::Error) -> CSVSchemaError {
        CSVSchemaError::IOError(err)
    }
}

impl std::fmt::Display for CSVSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CSVSchemaError::InvalidPath(e) => write!(f, "Invalid CSV path: {}", e),
            CSVSchemaError::InvalidTableName(e) => write!(f, "Invalid table name: {}", e),
            CSVSchemaError::InvalidColumnName(e) => write!(f, "Invalid column name: {}", e),
            CSVSchemaError::DuplicateColumn(e) => write!(f, "Duplicate column: {}", e),
            CSVSchemaError::DuplicateTable(e) => write!(f, "Duplicate table: {}", e),
            CSVSchemaError::CSVError(e) => write!(f, "CSV Error: {}", e),
            CSVSchemaError::IOError(e) => write!(f, "IO Error: {}", e),
            CSVSchemaError::UnknownDataType(e) => write!(f, "Unknown data type: {}", e),
            CSVSchemaError::UnknownForeignKey {
                table_name,
                column_name,
                foreign_table_name,
                foreign_column_name,
            } => write!(
                f,
                "Unknown foreign key: {}.{} -> {}.{}",
                table_name, column_name, foreign_table_name, foreign_column_name
            ),
            CSVSchemaError::InvalidTemporaryTableName(e) => {
                write!(f, "Invalid temporary table name: {}", e)
            }
            CSVSchemaError::EmptyColumn => write!(f, "Empty column"),
        }
    }
}

impl Error for CSVSchemaError {}

impl CSVSchemaError {
    pub fn unknown_data_type(data_type: &str) -> CSVSchemaError {
        CSVSchemaError::UnknownDataType(data_type.to_string())
    }
}
