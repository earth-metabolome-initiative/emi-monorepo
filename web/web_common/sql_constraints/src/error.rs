//! Submodule defining the error enumeration which may occur when applying
//! constraints.

mod constraint_error_info;
pub use constraint_error_info::ConstraintErrorInfo;

use crate::traits::ConstraintFailureInformation;

#[derive(Debug, thiserror::Error)]
/// Enumeration of possible errors that may occur when applying constraints.
pub enum Error {
    #[error("Table constraint violated: {0}")]
    /// Error indicating that a table constraint was violated.
    Table(Box<dyn ConstraintFailureInformation>),
    /// Unapplicable constraint error.
    #[error("Unapplicable constraint: {0}")]
    Unapplicable(String),
    #[error("Column constraint violated: {0}")]
    /// Error indicating that a column constraint was violated.
    Column(Box<dyn ConstraintFailureInformation>),
    #[error("Foreign key constraint violated: {0}")]
    /// Error indicating that a foreign key constraint was violated.
    ForeignKey(Box<dyn ConstraintFailureInformation>),
}
