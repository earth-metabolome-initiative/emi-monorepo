//! This module provides a trait for determining whether a grayscale image is
//! blurry.

/// Trait for determining whether a grayscale image is blurry.
pub trait IsBlurry {
    /// Returns whether the provided grayscale image is blurry.
    ///
    /// # Arguments
    /// * `sharpness_threshold`: The threshold below which the image is
    ///   considered blurry (optional, default: 5000.0).
    fn is_blurry(&self, sharpness_threshold: Option<f32>) -> bool;
}
