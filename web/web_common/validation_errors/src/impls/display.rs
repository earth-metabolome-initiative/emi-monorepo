//! Implementation of the `Display` trait for validation errors.

use std::fmt::{Display, Formatter, Result};

use crate::structs::{DoubleFieldError, SingleFieldError, ValidationError};

impl<FieldName: Display> Display for SingleFieldError<FieldName> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MustNotBeEmpty(field) => {
                write!(f, "Field `{field}` must not be empty")
            }
            Self::MustBeStrictlyPositive(field) => {
                write!(f, "Field `{field}` must be strictly positive (> 0)")
            }
            Self::MustBePositive(field) => {
                write!(f, "Field `{field}` must be positive (>= 0)")
            }
            Self::MustBeStrictlySmallerThan(field, value) => {
                write!(f, "Field `{field}` must be strictly smaller than {value}")
            }
            Self::MustBeSmallerThan(field, value) => {
                write!(f, "Field `{field}` must be smaller than or equal to {value}")
            }
            Self::MustBeStrictlyGreaterThan(field, value) => {
                write!(f, "Field `{field}` must be strictly greater than {value}")
            }
            Self::MustBeGreaterThan(field, value) => {
                write!(f, "Field `{field}` must be greater than or equal to {value}")
            }
            Self::Generic(field, error) => {
                write!(f, "Field `{field}`: {error}")
            }
        }
    }
}

impl<FieldName: Display> Display for DoubleFieldError<FieldName> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MustBeDistinct(field1, field2) => {
                write!(f, "Fields `{field1}` and `{field2}` must be distinct")
            }
            Self::MustBeStrictlySmallerThan(field1, field2) => {
                write!(f, "Field `{field1}` must be strictly smaller than field `{field2}`")
            }
            Self::MustBeSmallerThan(field1, field2) => {
                write!(f, "Field `{field1}` must be smaller than or equal to field `{field2}`")
            }
            Self::MustBeStrictlyGreaterThan(field1, field2) => {
                write!(f, "Field `{field1}` must be strictly greater than field `{field2}`")
            }
            Self::MustBeGreaterThan(field1, field2) => {
                write!(f, "Field `{field1}` must be greater than or equal to field `{field2}`")
            }
            Self::Generic(field1, field2, error) => {
                write!(f, "Fields `{field1}` and `{field2}`: {error}")
            }
        }
    }
}

impl<FieldName: Display> Display for ValidationError<FieldName> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::SingleField(error) => write!(f, "{error}"),
            Self::DoubleField(error) => write!(f, "{error}"),
        }
    }
}
