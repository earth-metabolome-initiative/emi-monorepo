//! Submodule providing the Operation trait.

use common_traits::prelude::{basic, Basic};

use crate::{operation_error::OperationError, outcome::Outcome};

/// Trait for operations.
pub trait Operation: Basic {
    /// The outcome type associated with the operation.
    type Outcome: Outcome<Operation = Self>;
    /// The error type associated with the operation.
    type Error: OperationError<Operation = Self>;

    /// Returns the identifier of the operation.
    fn id(&self) -> uuid::Uuid;

    #[cfg(feature = "backend")]
    /// Executes the operation.
    fn execute(
        self,
        connection: &mut diesel::PgConnection,
    ) -> impl std::future::Future<Output = Result<Self::Outcome, Self::Error>> + Send;
}

#[basic]
/// Generic operation.
pub struct GenericOperation<O> {
    /// The identifier of the operation.
    id: uuid::Uuid,
    /// The inner operation.
    operation: O,
}

impl<O> Operation for GenericOperation<O>
where
    O: Operation,
    <O as Operation>::Outcome: Outcome<Operation = Self>,
    <O as Operation>::Error: OperationError<Operation = Self>,
{
    type Outcome = <O as Operation>::Outcome;
    type Error = <O as Operation>::Error;

    fn id(&self) -> uuid::Uuid {
        self.id
    }

    #[cfg(feature = "backend")]
    fn execute(
        self,
        connection: &mut diesel::PgConnection,
    ) -> impl std::future::Future<Output = Result<Self::Outcome, Self::Error>> + Send {
        self.operation.execute(connection)
    }
}
