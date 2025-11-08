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
}
