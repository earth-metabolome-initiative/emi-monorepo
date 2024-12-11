//! Submodule defining an error enumeration for the custom constraints.
use std::fmt::{Display, Formatter};

use crate::{Column, Table};

#[derive(Debug)]
pub enum ConstraintError {
    UnexpectedNullableColumn(String),
    UnexpectedUppercaseColumn(String),
    UnexpectedUppercaseTable(String),
    NotForeignKeyColumn {
        table: Table,
        column: Column,
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
                write!(f, "Unexpected nullable column: {}", column_name)
            }
            ConstraintError::UnexpectedUppercaseColumn(column_name) => {
                write!(f, "Unexpected uppercase column: {}", column_name)
            }
            ConstraintError::UnexpectedUppercaseTable(table_name) => {
                write!(f, "Unexpected uppercase table: {}", table_name)
            }
            ConstraintError::NotForeignKeyColumn {
                table,
                column,
            } => {
                write!(
                    f,
                    "Column {} in table {} is not a foreign key column",
                    column.column_name, table.table_name
                )
            }
            ConstraintError::NotOfCorrectType {
                column_name,
                column_type,
                expected_column_type,
            } => {
                write!(
                    f,
                    "Column {} is of type {}, expected {}",
                    column_name, column_type, expected_column_type
                )
            }
            ConstraintError::DoesNotHaveSiblingColumn {
                table_name,
                column_name,
                sibling_column_name,
            } => {
                write!(
                    f,
                    "In table {}, if column {} is present, then column {} must also be present. It is currently missing.",
                    table_name, column_name, sibling_column_name
                )
            }
        }
    }
}
