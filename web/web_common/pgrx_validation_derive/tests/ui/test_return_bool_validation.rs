//! Submodule for testing compile failure of the `validation` attribute on functions which
//! return the wrong return type.
use pgrx_validation_derive::validation;

#[validation]
/// A simple validation function which returns the wrong error type should fail.
pub fn is_not_empty(_arg: &str) -> bool {
	true
}

/// The main for this test.
fn main() {}
