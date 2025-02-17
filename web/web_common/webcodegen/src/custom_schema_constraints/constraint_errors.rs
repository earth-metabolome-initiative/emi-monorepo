//! Submodule defining an error enumeration for the custom constraints.
use std::fmt::{Display, Formatter};

use crate::Column;

#[derive(Debug)]
/// Error type for custom schema constraints.
pub enum ConstraintError {
    /// The column is unexpectedly nullable.
    UnexpectedNullableColumn(String),
    /// The column name is unexpectedly uppercase.
    UnexpectedUppercaseColumn(String),
    /// The table name is unexpectedly uppercase.
    UnexpectedUppercaseTable(String),
    /// The column and foreign column types are incompatible.
    IncompatibleForeignType {
        /// The column
        column: Box<Column>,
        /// The foreign column
        foreign_column: Box<Column>,
    },
    /// The column is not a foreign key column.
    NotForeignKeyColumn {
        /// The name of the table
        table_name: String,
        /// The name of the column
        column_name: String,
    },
    /// The column is not of the expected type.
    NotOfCorrectType {
        /// The name of the column
        column_name: String,
        /// The type of the column
        column_type: String,
        /// The expected type of the column
        expected_column_type: String,
    },
    /// The column does not have a sibling column.
    DoesNotHaveSiblingColumn {
        /// The name of the table
        table_name: String,
        /// The name of the column
        column_name: String,
        /// The name of the sibling column
        sibling_column_name: String,
    },
}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ConstraintError::UnexpectedNullableColumn(column_name) => {
                write!(f, "Unexpected nullable column: {column_name}")
            }
            ConstraintError::UnexpectedUppercaseColumn(column_name) => {
                write!(f, "Unexpected uppercase column: {column_name}")
            }
            ConstraintError::UnexpectedUppercaseTable(table_name) => {
                write!(f, "Unexpected uppercase table: {table_name}")
            }
            ConstraintError::IncompatibleForeignType { column, foreign_column } => {
                write!(
                    f,
                    "Column {column_name} is of type {column_type}, foreign column {foreign_column_name} is of type {foreign_column_type}",
                    column_name = column.column_name,
                    column_type = column.raw_data_type(),
                    foreign_column_name = foreign_column.column_name,
                    foreign_column_type = foreign_column.raw_data_type(),
                )
            }
            ConstraintError::NotForeignKeyColumn { table_name, column_name } => {
                write!(f, "Column {column_name} in table {table_name} is not a foreign key column",)
            }
            ConstraintError::NotOfCorrectType {
                column_name,
                column_type,
                expected_column_type,
            } => {
                write!(
                    f,
                    "Column {column_name} is of type {column_type}, expected {expected_column_type}",
                )
            }
            ConstraintError::DoesNotHaveSiblingColumn {
                table_name,
                column_name,
                sibling_column_name,
            } => {
                write!(
                    f,
                    "In table {table_name}, if column {column_name} is present, then column {sibling_column_name} must also be present. It is currently missing.",
                )
            }
        }
    }
}
