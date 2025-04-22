//! Submodule providing the `IsDark` trait which determines whether an image is
//! too underexposed.

/// Trait providing the `is_dark` method to determine whether an image is too
/// underexposed.
pub trait IsDark {
    /// Error type for the trait.
    type Error: core::error::Error;

    /// Returns whether the image is too underexposed.
    ///
    /// # Arguments
    /// * `threshold` - The threshold for the luma value of a pixel to be
    ///   considered dark. If not provided, the default value is 0.1.
    ///
    /// # Errors
    ///
    /// * If the provided threshold is not in the range [0.0, 1.0].
    fn is_dark(&self, threshold: Option<f32>) -> Result<bool, Self::Error>;
}
