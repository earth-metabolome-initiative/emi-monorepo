//! Tests whether the derive macro raises the expected error when the function name is camelcased.

use pgrx_validation_derive::validation;

#[validation]
/// A CamelCased validation function should fail.
pub fn MustNotBeEmpty(_arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

/// The main for this test.
fn main() {}
