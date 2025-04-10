//! This module defines a trait for calculating the rate of repeated colors in an image.

/// Trait for calculating the rate of repeated colors in an image.
pub trait RepeatedColors {
    /// Returns the rate of repeated colors in the provided image.
    ///
    /// # Arguments
    /// * `number_of_colors`: The number of most common colors to consider.
    fn get_repeated_colors_rate(&self, number_of_colors: usize) -> f32;
}