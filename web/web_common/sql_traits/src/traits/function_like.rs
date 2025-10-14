//! Submodule providing a trait for describing SQL Function-like entities.

/// A trait for describing SQL Function-like entities.
pub trait FunctionLike {
	/// The name of the function.
	fn name(&self) -> &str;
}