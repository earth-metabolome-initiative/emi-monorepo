//! Submodule providing the Outcome trait.

use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::{Connection, Operation};

/// Trait for outcomes.
pub trait Outcome<C: Connection>: Serialize + DeserializeOwned {
	/// The operation type associated with the outcome.
	type Operation: Operation<C, Outcome = Self>;

	/// Returns the operation associated with the outcome.
	fn operation(&self) -> &Self::Operation;

	/// Returns the identifier of the operation.
	fn id(&self) -> uuid::Uuid {
		self.operation().id()
	}
}