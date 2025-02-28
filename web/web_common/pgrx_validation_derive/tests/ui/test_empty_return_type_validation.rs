//! Submodule for testing compile failure of the `validation` attribute on functions which
//! do not return nothing.
use pgrx_validation_derive::validation;

#[validation]
/// A simple validation function which does not return anything should fail.
pub fn is_not_empty(_arg: &str) {}

/// The main for this test.
fn main() {}
