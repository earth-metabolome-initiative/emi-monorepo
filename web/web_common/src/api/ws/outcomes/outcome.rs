//! Submodule providing the OutcomeMessage enum and the Outcome trait.


pub trait Outcome {
	
	type Operation;

	/// Returns the identifier of the operation.
	fn id(&self) -> uuid::Uuid;
}

/// Enumeration of all possible operations.
#[derive(Debug)]
pub enum OutcomeMessage {
	/// 
	Insert
}