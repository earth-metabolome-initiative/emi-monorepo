//! Submodule defining the structs used for validation errors.

use crate::structs::{DoubleFieldError, SingleFieldError};

#[derive(Debug)]
/// Enumeration of errors that can occur during validation.
pub enum ValidationError<FieldName> {
    /// Single field errors.
    SingleField(SingleFieldError<FieldName>),
    /// Double field errors.
    DoubleField(DoubleFieldError<FieldName>),
}
