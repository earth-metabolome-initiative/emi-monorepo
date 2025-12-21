//! Submodule defining the `ExternalFunction` struct, which
//! contains minimal information about a function and its crate
//! of provenance.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Minimal information about an external function.
pub struct ExternalFunction {
    /// Name of the external function.
    name: String,
    /// Pah to the external function.
    path: syn::Path,
}

impl ExternalFunction {
	/// Creates a new `ExternalFunction`.
	#[must_use]
	pub fn new(name: &str, path: syn::Path) -> Self {
		Self {
			name: name.to_string(),
			path,
		}
	}

	/// Returns the name of the external function.
	#[inline]
	#[must_use]
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Returns the path to the external function.
	#[inline]
	#[must_use]
	pub fn path(&self) -> &syn::Path {
		&self.path
	}
}