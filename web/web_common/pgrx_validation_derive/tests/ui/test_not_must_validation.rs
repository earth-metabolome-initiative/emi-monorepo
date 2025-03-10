//! Submodule for testing compile failure of the `validation` attribute on functions which
//! does not adhere to the standard naming conventions.
use pgrx_validation_derive::validation;

#[validation]
/// A validation which does not adhere to the standard naming conventions should fail.
pub fn other_must_not_be_empty(_arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

/// The main for this test.
fn main() {}
