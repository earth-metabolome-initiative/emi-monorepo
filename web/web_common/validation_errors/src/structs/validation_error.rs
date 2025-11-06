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
