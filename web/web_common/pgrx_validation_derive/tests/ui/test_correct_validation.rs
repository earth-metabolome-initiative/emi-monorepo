//! Submodule for testing compile success of the `validation` attribute on
//! functions which return the correct return type.
use pgrx_validation_derive::validation;

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_be_another_empty(_arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

#[validation]
/// A simple validation function which returns the correct error type should
/// pass.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_be_empty(arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    must_not_be_another_empty(arg)?;
    Ok(())
}

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_contain_another_empty(
    _arg: &str,
) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_contain_another_empty(_arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    Ok(())
}

#[validation]
/// A simple validation function which returns the correct error type should
/// pass.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_contain_empty(arg: &str) -> Result<(), validation_errors::SingleFieldError> {
    must_not_be_another_empty(arg)?;
    must_contain_another_empty(arg)?;
    must_not_contain_another_empty(arg)?;
    Ok(())
}

/// The main for this test.
fn main() {}
