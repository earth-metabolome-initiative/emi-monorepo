//! Submodule providing the `IsLight` trait which determines whether an image is
//! too underexposed.

/// Trait providing the `is_Light` method to determine whether an image is too
/// underexposed.
pub trait IsLight {
    /// Error type for the trait.
    type Error: core::error::Error;

    /// Returns whether the image is too underexposed.
    ///
    /// # Arguments
    /// * `threshold` - The threshold for the luma value of a pixel to be
    ///   considered light. If not provided, the default value is 0.8.
    /// 
    /// # Errors
    /// 
    /// * If the provided threshold is not in the range [0.0, 1.0].
    fn is_light(&self, threshold: Option<f32>) -> Result<bool, Self::Error>;
}
