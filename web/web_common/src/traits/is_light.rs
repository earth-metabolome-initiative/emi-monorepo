//! Submodule providing the `IsLight` trait which determines whether an image is too underexposed.
/// Trait providing the `is_Light` method to determine whether an image is too underexposed.
pub trait IsLight {
    /// Returns whether the image is too underexposed.
    ///
    /// # Arguments
    /// * `threshold` - The threshold for the luma value of a pixel to be considered light. If not provided, the default value is 0.8.
    ///
    fn is_light(&self, threshold: Option<f32>) -> bool;
}

impl IsLight for image::GrayImage {
    fn is_light(&self, threshold: Option<f32>) -> bool {
        let threshold = threshold.unwrap_or(0.8);
        let threshold_u8 = (threshold * 255.0) as u8;
        let mut light_pixels = 0;
        for pixel in self.pixels() {
            if pixel.0[0] > threshold_u8 {
                light_pixels += 1;
            }
        }
        light_pixels as f32 / self.width() as f32 / self.height() as f32 > 0.5
    }
}
