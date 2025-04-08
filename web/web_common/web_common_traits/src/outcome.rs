//! Submodule providing the Outcome trait.

use common_traits::basic::Basic;

use crate::prelude::Operation;

/// Trait for outcomes.
pub trait Outcome: Basic {
    /// The operation type associated with the outcome.
    type Operation: Operation<Outcome = Self>;

    /// Returns the operation associated with the outcome.
    fn operation(&self) -> &Self::Operation;

    /// Returns the identifier of the operation.
    fn id(&self) -> rosetta_uuid::Uuid {
        self.operation().id()
    }
}

/// Trait for outcomes that can be created from an execution result
pub trait FromExecution<E>: Outcome {
    /// Creates a new outcome from an execution result.
    fn from_outcome(operation: Self::Operation, outcome: E) -> Self;
}
