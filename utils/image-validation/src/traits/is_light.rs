//! Submodule providing the `IsLight` trait which determines whether an image is
//! too underexposed.

/// Trait providing the `is_Light` method to determine whether an image is too
/// underexposed.
pub trait IsLight {
    /// Returns whether the image is too underexposed.
    ///
    /// # Arguments
    /// * `threshold` - The threshold for the luma value of a pixel to be
    ///   considered light. If not provided, the default value is 0.8.
    fn is_light(&self, threshold: Option<f32>) -> bool;
}
