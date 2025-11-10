//! Submodule defining the structs used for validation errors.

use crate::structs::{DoubleFieldError, SingleFieldError, Unspecified};

#[derive(Debug)]
/// Enumeration of errors that can occur during validation.
pub enum ValidationError<FieldName = Unspecified> {
    /// Single field errors.
    SingleField(SingleFieldError<FieldName>),
    /// Double field errors.
    DoubleField(DoubleFieldError<FieldName>),
}

impl<FieldName> ValidationError<FieldName> {
    /// Creates a new validation error for an empty field.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that is empty.
    ///
    /// # Returns
    ///
    /// A `ValidationError` indicating that the specified field is empty.
    pub fn empty(field: FieldName) -> Self {
        ValidationError::SingleField(SingleFieldError::MustNotBeEmpty(field))
    }

    /// Creates a new validation error for two fields that must not be equal.
    ///
    /// # Arguments
    ///
    /// * `left_field` - The name of the first field.
    /// * `right_field` - The name of the second field.
    pub fn equal(left_field: FieldName, right_field: FieldName) -> Self {
        ValidationError::DoubleField(DoubleFieldError::MustBeDistinct(left_field, right_field))
    }

    /// Creates a new validation error for a field who should be smaller than or
    /// equal to another field.
    ///
    /// # Arguments
    ///
    /// * `smaller_field` - The name of the field that should be smaller.
    /// * `greater_field` - The name of the field that should be greater.
    pub fn smaller_than(smaller_field: FieldName, greater_field: FieldName) -> Self {
        ValidationError::DoubleField(DoubleFieldError::MustBeSmallerThan(
            smaller_field,
            greater_field,
        ))
    }

    /// Creates a new validation error for a field who should be smaller than or
    /// equal to a provided value.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that should be smaller.
    /// * `value` - The value that the field should be smaller than or equal to.
    pub fn smaller_than_value(field: FieldName, value: f64) -> Self {
        ValidationError::SingleField(SingleFieldError::MustBeSmallerThan(field, value))
    }

    /// Creates a new validation error for a field who should be greater than a
    /// another field.
    ///
    /// # Arguments
    ///
    /// * `greater_field` - The name of the field that should be greater.
    /// * `smaller_field` - The name of the field that should be smaller.
    pub fn greater_than(greater_field: FieldName, smaller_field: FieldName) -> Self {
        ValidationError::DoubleField(DoubleFieldError::MustBeGreaterThan(
            greater_field,
            smaller_field,
        ))
    }

    /// Creates a new validation error for a field who should be greater than or
    /// equal to a provided value.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that should be greater.
    /// * `value` - The value that the field should be greater than or equal to.
    pub fn greater_than_value(field: FieldName, value: f64) -> Self {
        ValidationError::SingleField(SingleFieldError::MustBeGreaterThan(field, value))
    }

    /// Creates a new validation error for a field who should be strictly
    /// smaller than another field.
    ///
    /// # Arguments
    ///
    /// * `smaller_equal_field` - The name of the field that should be strictly
    ///   smaller than another field.
    /// * `greater_field` - The name of the field that should be greater.
    pub fn strictly_smaller_than(smaller_equal_field: FieldName, greater_field: FieldName) -> Self {
        ValidationError::DoubleField(DoubleFieldError::MustBeStrictlySmallerThan(
            smaller_equal_field,
            greater_field,
        ))
    }

    /// Creates a new validation error for a field who should be strictly
    /// smaller than a provided value.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that should be strictly smaller.
    /// * `value` - The value that the field should be strictly smaller than.
    pub fn strictly_smaller_than_value(field: FieldName, value: f64) -> Self {
        ValidationError::SingleField(SingleFieldError::MustBeStrictlySmallerThan(field, value))
    }

    /// Creates a new validation error for a field who should be strictly
    /// greater than another field.
    ///
    /// # Arguments
    ///
    /// * `greater_equal_field` - The name of the field that should be strictly
    ///   greater than another field.
    /// * `smaller_field` - The name of the field that should be smaller.
    pub fn strictly_greater_than(greater_equal_field: FieldName, smaller_field: FieldName) -> Self {
        ValidationError::DoubleField(DoubleFieldError::MustBeStrictlyGreaterThan(
            greater_equal_field,
            smaller_field,
        ))
    }

    /// Creates a new validation error for a field who should be strictly
    /// greater than a provided value.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field that should be strictly greater.
    /// * `value` - The value that the field should be strictly greater than.
    pub fn strictly_greater_than_value(field: FieldName, value: f64) -> Self {
        ValidationError::SingleField(SingleFieldError::MustBeStrictlyGreaterThan(field, value))
    }
}
