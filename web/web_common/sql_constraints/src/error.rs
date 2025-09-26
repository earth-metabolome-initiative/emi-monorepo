//! Submodule defining the error enumeration which may occur when applying constraints.

use crate::traits::ConstraintFailureInformation;

/// Enumeration of possible errors that may occur when applying constraints.
pub enum Error {
    /// Error indicating that a table constraint was violated.
    Table(Box<dyn ConstraintFailureInformation>),
}
