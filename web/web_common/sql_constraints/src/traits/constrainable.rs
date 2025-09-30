//! Submodule defining the `Constrainable` trait, which defines an object that can be
//! constrained by SQL constraints.

/// Trait for types that define an object which can be constrained by SQL constraints.
pub trait Constrainable {
	/// Returns the name of the object.
	fn context_name(&self) -> String;
}
