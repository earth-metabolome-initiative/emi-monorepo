//! Submodule defining an error enumeration for the custom constraints.
use std::fmt::{Display, Formatter};


pub enum ConstraintError {
    UnexpectedNullableColumn(String),
    UnexpectedUppercaseColumn(String),
    UnexpectedUppercaseTable(String),
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
        }
    }
}