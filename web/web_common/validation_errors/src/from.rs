//! Implementation of the `From` trait for `ValidationErrors`.

use std::convert::Infallible;

use crate::{DoubleFieldError, Error, SingleFieldError};

impl From<Infallible> for Error {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to Error.")
    }
}

impl From<Infallible> for SingleFieldError {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to SingleFieldError.")
    }
}

impl From<Infallible> for DoubleFieldError {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to DoubleFieldError.")
    }
}

impl From<cas_codes::errors::Error> for SingleFieldError {
    fn from(error: cas_codes::errors::Error) -> Self {
        Self::InvalidCasCode((), error)
    }
}

impl From<cas_codes::errors::Error> for Error {
    fn from(error: cas_codes::errors::Error) -> Self {
        Self::SingleField(error.into())
    }
}

impl From<molecular_formulas::errors::Error> for SingleFieldError {
    fn from(error: molecular_formulas::errors::Error) -> Self {
        Self::InvalidMolecularFormula((), error)
    }
}

impl From<molecular_formulas::errors::Error> for Error {
    fn from(error: molecular_formulas::errors::Error) -> Self {
        Self::SingleField(error.into())
    }
}

impl From<media_types::errors::Error> for SingleFieldError {
    fn from(error: media_types::errors::Error) -> Self {
        Self::InvalidMediaType((), error)
    }
}
