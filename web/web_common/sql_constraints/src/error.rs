//! Submodule defining the error enumeration which may occur when applying
//! constraints.

mod constraint_error_info;
pub use constraint_error_info::ConstraintErrorInfo;

use crate::traits::ConstraintFailureInformation;

#[derive(Debug)]
/// Enumeration of possible errors that may occur when applying constraints.
pub enum Error {
    /// Error indicating that a table constraint was violated.
    Table(Box<dyn ConstraintFailureInformation>),
    /// Error indicating that a column constraint was violated.
    Column(Box<dyn ConstraintFailureInformation>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Table(info) => write!(f, "Table constraint violation: {info}"),
            Error::Column(info) => write!(f, "Column constraint violation: {info}"),
        }
    }
}

impl std::error::Error for Error {}
