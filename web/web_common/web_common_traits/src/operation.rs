//! Submodule providing the Operation trait.

use serde::{de::DeserializeOwned, Serialize};

use crate::{outcome::Outcome, prelude::Connection};

/// Trait for operations.
pub trait Operation<C: Connection>: Serialize + DeserializeOwned {
    /// The outcome type associated with the operation.
    type Outcome: Outcome<C, Operation = Self>;
    /// The error type associated with the operation.
    type Error: std::error::Error;

    /// Returns the identifier of the operation.
    fn id(&self) -> uuid::Uuid;

    /// Executes the operation.
    fn execute(
        &self,
        connection: &mut C,
    ) -> impl std::future::Future<Output = Result<Self::Outcome, Self::Error>> + Send;
}
