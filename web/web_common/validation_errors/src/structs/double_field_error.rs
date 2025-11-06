//! Submodule defining the structs used for double field validation errors.

use crate::{structs::Unspecified, traits::ValidationErrorLike};

#[derive(Debug)]
/// Enumeration of errors that can occur during validation of two fields.
pub enum DoubleFieldError<FieldName = Unspecified> {
    /// The provided entries should be distinct.
    MustBeDistinct(FieldName, FieldName),
    /// The provided left entry must be strictly smaller than the right entry.
    MustBeStrictlySmallerThan(FieldName, FieldName),
    /// The provided left entry must be smaller than the right entry.
    MustBeSmallerThan(FieldName, FieldName),
    /// The provided left entry must be strictly greater than the right entry.
    MustBeStrictlyGreaterThan(FieldName, FieldName),
    /// The provided left entry must be greater than the right entry.
    MustBeGreaterThan(FieldName, FieldName),
    /// Some third-party validation error.
    Generic(FieldName, FieldName, Box<dyn ValidationErrorLike>),
}
