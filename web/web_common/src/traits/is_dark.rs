//! Submodule providing the `IsDark` trait which determines whether an image is too underexposed.

/// Trait providing the `is_dark` method to determine whether an image is too underexposed.
pub trait IsDark {
    /// Returns whether the image is too underexposed.
    ///
    /// # Arguments
    /// * `threshold` - The threshold for the luma value of a pixel to be considered dark. If not provided, the default value is 0.3.
    ///
    fn is_dark(&self, threshold: Option<f32>) -> bool;
}

impl IsDark for image::GrayImage {
    fn is_dark(&self, threshold: Option<f32>) -> bool {
        let threshold = threshold.unwrap_or(0.3);
        let threshold_u8 = (threshold * 255.0) as u8;
        let mut dark_pixels = 0;
        for pixel in self.pixels() {
            if pixel.0[0] < threshold_u8 {
                dark_pixels += 1;
            }
        }
        dark_pixels as f32 / self.width() as f32 / self.height() as f32 > 0.5
    }
}