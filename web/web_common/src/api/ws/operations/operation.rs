//! Submodule providing the OperationMessage enum and the Operation trait.


pub trait Operation {
	type Outcome;

	/// Returns the identifier of the operation.
	fn id(&self) -> uuid::Uuid;
}

/// Enumeration of all possible operations.
#[derive(Debug)]
pub enum OperationMessage {
	/// 
	Insert
}