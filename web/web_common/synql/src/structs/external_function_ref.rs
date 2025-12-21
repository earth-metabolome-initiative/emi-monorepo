//! Submodule defining the `ExternalFunctionRef` struct, which
//! contains minimal information about a function and its crate
//! of provenance.

use crate::structs::{ExternalCrate, ExternalFunction};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Minimal information about an external function.
pub struct ExternalFunctionRef<'workspace> {
	/// Reference to the external function.
	external_function: &'workspace ExternalFunction,
	/// Crate where the external function is defined.
	crate_ref: &'workspace ExternalCrate,
}

impl<'workspace> ExternalFunctionRef<'workspace> {
	/// Creates a new `ExternalFunctionRef`.
	#[must_use]
	pub fn new(
		crate_ref: &'workspace ExternalCrate,
		external_function: &'workspace ExternalFunction,
	) -> Self {
		Self {
			external_function,
			crate_ref,
		}
	}

	/// Returns the name of the external function.
	#[inline]
	#[must_use]
	pub fn name(&self) -> &str {
		self.external_function.name()
	}

	/// Returns the path to the external function.
	#[inline]
	#[must_use]
	pub fn path(&self) -> &syn::Path {
		self.external_function.path()
	}

	/// Returns the crate where the external function is defined.
	#[inline]
	#[must_use]
	pub fn crate_ref(&self) -> &ExternalCrate {
		self.crate_ref
	}
}