//! A simple struct for generic validation errors to avoid code duplication for
//! trivial error types.

use std::error::Error;

use crate::traits::ValidationErrorLike;

#[derive(Debug)]
/// A generic error struct that can be used for simple validation errors that
/// do not require additional context.
pub struct GenericError {
    message: String,
}

impl From<String> for GenericError {
    fn from(message: String) -> Self {
        GenericError { message }
    }
}

impl<'a> From<&'a str> for GenericError {
    fn from(message: &'a str) -> Self {
        GenericError::from(message.to_owned())
    }
}

impl Error for GenericError {}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ValidationErrorLike for GenericError {}
