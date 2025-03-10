//! Submodule for testing compile failure when the `validation` attribute is used on functions
//! which do not have a doc comment.
use pgrx_validation_derive::validation;

#[validation]
pub fn must_not_be_empty(_arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

/// The main for this test.
fn main() {}
