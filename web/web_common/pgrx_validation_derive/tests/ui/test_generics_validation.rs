//! Submodule for testing compile failure of the `validation` attribute on functions with generics.
use pgrx_validation_derive::validation;

#[validation]
/// A simple validation function with generics which should fail.
pub fn is_not_empty<S: ToString>(string: S) -> Result<(), String> {
    if string.to_string().is_empty() {
        Err("String is empty".to_string())
    } else {
        Ok(())
    }
}

/// The main for this test.
fn main() {}
