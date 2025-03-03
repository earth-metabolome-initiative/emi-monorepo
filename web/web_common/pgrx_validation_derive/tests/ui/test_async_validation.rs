//! Submodule for testing compile failure of the `validation` attribute on functions which
//! have an `async` signature.
use pgrx_validation_derive::validation;

#[validation]
/// An async validation function should not compile.
pub async fn must_not_be_empty(_arg: &str) -> Result<(), validation_errors::Error> {
    Ok(())
}

/// The main for this test.
fn main() {}
