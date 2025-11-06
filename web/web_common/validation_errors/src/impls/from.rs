//! Implementation of the `From` trait for `ValidationErrors`.

use std::convert::Infallible;

use crate::prelude::{DoubleFieldError, SingleFieldError, ValidationError};

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
