//! Submodule for testing compile success of the `validation` attribute on functions which
//! return the correct return type.
use pgrx_validation_derive::validation;

#[validation]
/// A simple validation function which returns the correct error type should pass.
pub fn is_not_empty(_arg: &str) -> Result<(), validation_errors::Error> {
    Ok(())
}

/// The main for this test.
fn main() {}
