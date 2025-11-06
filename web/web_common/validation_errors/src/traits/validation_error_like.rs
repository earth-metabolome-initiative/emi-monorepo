//! Submodule defining the `ValidationErrorLike` trait.

use std::error::Error;

/// Trait for types that behave like validation errors.
pub trait ValidationErrorLike: Error {}
