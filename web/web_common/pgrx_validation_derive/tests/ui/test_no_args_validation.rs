//! Submodule for testing compile failure of the `validation` attribute on functions which
//! does not have any arguments.
use pgrx_validation_derive::validation;

#[validation]
/// A simple validation function which does not receive any arguments should fail.
pub fn is_not_empty() -> Result<(), String> {
	Ok(())
}

/// The main for this test.
fn main() {}
