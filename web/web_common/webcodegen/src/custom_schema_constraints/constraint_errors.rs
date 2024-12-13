//! Submodule defining an error enumeration for the custom constraints.
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConstraintError {
    UnexpectedNullableColumn(String),
    UnexpectedUppercaseColumn(String),
    UnexpectedUppercaseTable(String),
    NotForeignKeyColumn {
        table_name: String,
        column_name: String,
    },
    NotOfCorrectType {
        column_name: String,
        column_type: String,
        expected_column_type: String,
    },
    DoesNotHaveSiblingColumn {
        table_name: String,
        column_name: String,
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
            ConstraintError::NotForeignKeyColumn {
                table_name,
                column_name,
            } => {
                write!(
                    f,
                    "Column {column_name} in table {table_name} is not a foreign key column",
                )
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
