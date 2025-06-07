//! Submodule providing the Operation trait.

use crate::{operation_error::OperationError, outcome::Outcome};

/// Trait for operations.
pub trait Operation {
    /// The outcome type associated with the operation.
    type Outcome: Outcome<Operation = Self>;
    /// The error type associated with the operation.
    type Error: OperationError<Operation = Self>;

    /// Returns the identifier of the operation.
    fn id(&self) -> rosetta_uuid::Uuid;
}
