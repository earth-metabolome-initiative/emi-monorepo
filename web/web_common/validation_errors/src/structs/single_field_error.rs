//! Submodule defining the structs used for single field validation errors.

use crate::traits::ValidationErrorLike;

#[derive(Debug)]
/// Enumeration of errors that can occur during validation of a single field.
pub enum SingleFieldError<FieldName> {
    /// The provided text is empty.
    EmptyText(FieldName),
    /// The numeric value is not strictly positive (0.0, ...]
    MustBeStrictlyPositive(FieldName),
    /// The numeric value is not positive [0.0, ...]
    MustBePositive(FieldName),
    /// The scalar is not strictly greater than the expected amount.
    MustBeStrictlySmallerThan(FieldName, f64),
    /// The scalar is not smaller than the expected amount.
    MustBeSmallerThan(FieldName, f64),
    /// The scalar is not strictly greater than the expected amount.
    MustBeStrictlyGreaterThan(FieldName, f64),
    /// The scalar is not greater than the expected amount.
    MustBeGreaterThan(FieldName, f64),
    /// Some third-party validation error.
    Generic(FieldName, Box<dyn ValidationErrorLike>),
}
