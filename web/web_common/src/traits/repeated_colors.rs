//! Submodule providing the `RepeatedColors` trait.
use image::GenericImageView;
use image::Pixel;

pub trait RepeatedColors {
    /// Returns the rate of repeated colors in the provided image.
    ///
    /// # Arguments
    /// * `number_of_colors`: The number of most common colors to consider.
    fn get_repeated_colors_rate(&self, number_of_colors: usize) -> f32;
}

impl RepeatedColors for image::DynamicImage {
    fn get_repeated_colors_rate(&self, number_of_colors: usize) -> f32 {
        use std::collections::HashMap;
        let mut color_count: HashMap<image::Rgb<u8>, u32> = HashMap::new();
        let (width, height) = self.dimensions();

        for (_, _, pixel) in self.pixels() {
            *color_count.entry(pixel.to_rgb()).or_insert(0) += 1;
        }

        let mut counts: Vec<u32> = color_count.values().copied().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

        let top_ten_sum: u32 = counts.iter().take(number_of_colors).sum();
        let total_pixels = width * height;
        top_ten_sum as f32 / total_pixels as f32
    }
}
