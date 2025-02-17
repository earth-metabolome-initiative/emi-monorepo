//! Submodule providing errors that may occur during CSV schema processing.

use std::error::Error;

#[derive(Debug)]
/// Enum representing errors that may occur during CSV schema processing.
pub enum CSVSchemaError {
    /// Error indicating an invalid CSV path.
    InvalidPath(String),
    /// Error indicating an invalid table name.
    InvalidTableName(String),
    /// Error indicating an invalid column name.
    InvalidColumnName(String),
    /// Error indicating a duplicate column.
    DuplicateColumn {
        /// The column name.
        column_name: Option<String>,
        /// The table name.
        table_name: String,
    },
    /// Error indicating a duplicate table.
    DuplicateTable(String),
    /// Error indicating a CSV error.
    CSVError(csv::Error),
    /// Error indicating an IO error.
    IOError(std::io::Error),
    /// Error indicating an unknown data type.
    UnknownDataType(String),
    /// Error indicating an unknown foreign key.
    UnknownForeignKey {
        /// The table name.
        table_name: String,
        /// The column with the foreign key.
        column_name: String,
        /// The foreign table name.
        foreign_table_name: String,
        /// The foreign column name.
        foreign_column_name: String,
    },
    /// Error indicating a loop in the foreign key chain.
    Loop(String),
    /// Error indicating an invalid temporary table name.
    InvalidTemporaryTableName(String),
    /// Error indicating an empty column.
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
            CSVSchemaError::InvalidPath(e) => write!(f, "Invalid CSV path: {e}"),
            CSVSchemaError::InvalidTableName(e) => write!(f, "Invalid table name: {e}"),
            CSVSchemaError::InvalidColumnName(e) => write!(f, "Invalid column name: {e}"),
            CSVSchemaError::DuplicateColumn { column_name, table_name } => {
                write!(f, "Duplicate column: {column_name:?} in table {table_name}")
            }
            CSVSchemaError::DuplicateTable(e) => write!(f, "Duplicate table: {e}"),
            CSVSchemaError::CSVError(e) => write!(f, "CSV Error: {e}"),
            CSVSchemaError::IOError(e) => write!(f, "IO Error: {e}"),
            CSVSchemaError::UnknownDataType(e) => write!(f, "Unknown data type: {e}"),
            CSVSchemaError::UnknownForeignKey {
                table_name,
                column_name,
                foreign_table_name,
                foreign_column_name,
            } => write!(
                f,
                "Unknown foreign key: {table_name}.{column_name} -> {foreign_table_name}.{foreign_column_name}",
            ),
            CSVSchemaError::Loop(table) => {
                write!(f, "Loop in foreign key starting from table: {table:?}")
            }
            CSVSchemaError::InvalidTemporaryTableName(e) => {
                write!(f, "Invalid temporary table name: {e}")
            }
            CSVSchemaError::EmptyColumn => write!(f, "Empty column"),
        }
    }
}

impl Error for CSVSchemaError {}
