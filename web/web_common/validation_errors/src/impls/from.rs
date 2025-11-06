//! Implementation of the `From` trait for `ValidationErrors`.

use std::convert::Infallible;

use crate::{
    prelude::{DoubleFieldError, SingleFieldError, ValidationError},
    structs::{GenericError, Unspecified},
};

impl<FieldName> From<Infallible> for ValidationError<FieldName> {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to Error.")
    }
}

impl<FieldName> From<Infallible> for SingleFieldError<FieldName> {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to SingleFieldError.")
    }
}

impl<FieldName> From<Infallible> for DoubleFieldError<FieldName> {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to DoubleFieldError.")
    }
}

impl From<GenericError> for SingleFieldError<Unspecified> {
    fn from(error: GenericError) -> Self {
        SingleFieldError::Generic(Unspecified, Box::new(error))
    }
}

impl From<GenericError> for DoubleFieldError<Unspecified> {
    fn from(error: GenericError) -> Self {
        DoubleFieldError::Generic(Unspecified, Unspecified, Box::new(error))
    }
}

impl From<String> for SingleFieldError<Unspecified> {
    fn from(message: String) -> Self {
        SingleFieldError::Generic(Unspecified, Box::new(GenericError::from(message)))
    }
}

impl From<String> for DoubleFieldError<Unspecified> {
    fn from(message: String) -> Self {
        DoubleFieldError::Generic(Unspecified, Unspecified, Box::new(GenericError::from(message)))
    }
}

impl<'a> From<&'a str> for SingleFieldError<Unspecified> {
    fn from(message: &'a str) -> Self {
        SingleFieldError::Generic(Unspecified, Box::new(GenericError::from(message)))
    }
}

impl<'a> From<&'a str> for DoubleFieldError<Unspecified> {
    fn from(message: &'a str) -> Self {
        DoubleFieldError::Generic(Unspecified, Unspecified, Box::new(GenericError::from(message)))
    }
}
