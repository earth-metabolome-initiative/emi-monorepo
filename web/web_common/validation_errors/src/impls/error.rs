//! Implementation of the `Error` trait for validation errors.

use std::error::Error;

use crate::{
    structs::{DoubleFieldError, SingleFieldError, ValidationError},
    traits::ValidationErrorLike,
};

impl<FieldName: std::fmt::Debug + std::fmt::Display> Error for SingleFieldError<FieldName> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SingleFieldError::Generic(_, error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl<FieldName: std::fmt::Debug + std::fmt::Display> Error for DoubleFieldError<FieldName> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DoubleFieldError::Generic(_, _, error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl<FieldName: std::fmt::Debug + std::fmt::Display> Error for ValidationError<FieldName> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ValidationError::SingleField(error) => error.source(),
            ValidationError::DoubleField(error) => error.source(),
        }
    }
}

// Implement ValidationErrorLike for all our error types
impl<FieldName: std::fmt::Debug + std::fmt::Display> ValidationErrorLike
    for SingleFieldError<FieldName>
{
}
impl<FieldName: std::fmt::Debug + std::fmt::Display> ValidationErrorLike
    for DoubleFieldError<FieldName>
{
}
impl<FieldName: std::fmt::Debug + std::fmt::Display> ValidationErrorLike
    for ValidationError<FieldName>
{
}
