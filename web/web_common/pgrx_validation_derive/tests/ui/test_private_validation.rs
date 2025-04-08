//! Submodule for testing compile failure of the `validation` attribute on private functions.
use pgrx_validation_derive::validation;

#[validation]
/// A simple private validation function which should fail.
fn is_not_empty(string: &str) -> Result<(), String> {
    if string.is_empty() {
        Err("String is empty".to_string())
    } else {
        Ok(())
    }
}

/// The main for this test.
fn main() {}
