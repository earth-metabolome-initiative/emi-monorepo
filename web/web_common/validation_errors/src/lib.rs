//! Crate providing common validation errors.

mod impls;
mod structs;
mod traits;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
    pub use crate::{
        structs::{DoubleFieldError, SingleFieldError, ValidationError},
        traits::{ReplaceFieldName, ValidationErrorLike},
    };
}
