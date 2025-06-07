//! Trait characterizing an error that may occur during the execution of an
//! operation.

use core::error::Error;

use crate::prelude::Operation;

/// Trait for operation errors.
pub trait OperationError: Error {
    /// The operation associated to this error.
    type Operation: Operation<Error = Self>;

    /// Returns a reference to the underlying operation.
    fn operation(&self) -> &Self::Operation;

    /// Returns the identifier of the operation.
    fn id(&self) -> rosetta_uuid::Uuid {
        self.operation().id()
    }
}

/// Trait for operation errors that can be created from a supported error.
pub trait FromError<E: Error>: OperationError {
    /// Creates a new operation error from an operation and a supported error.
    fn from_error(operation: Self::Operation, error: E) -> Self;
}
