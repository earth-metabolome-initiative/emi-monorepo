//! Trait for checking if an image is transparent.

/// Trait for checking if an image is transparent.
pub trait IsTransparent {
    /// Returns whether the provided image is transparent.
    fn is_transparent(&self) -> bool;
}
