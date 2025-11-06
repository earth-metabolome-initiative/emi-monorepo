//! Submodule defining the structs used for validation errors.

mod double_field_error;
pub use double_field_error::DoubleFieldError;
mod single_field_error;
pub use single_field_error::SingleFieldError;
mod validation_error;
pub use validation_error::ValidationError;
