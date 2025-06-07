//! Implementation of traits for [`DynamicImage`](image::DynamicImage).

use image::{GenericImageView, Pixel};
use rustfft::num_traits::float::TotalOrder;

use crate::traits::{has_repeated_colors::RepeatedColors, is_transparent::IsTransparent};

impl IsTransparent for image::DynamicImage {
    fn is_transparent(&self) -> bool {
        use image::ColorType::{La8, La16, Rgba8, Rgba16};
        match self.color() {
            La8 | La16 | Rgba8 | Rgba16 => self.pixels().any(|(_, _, pixel)| pixel.0[3] < 220),
            _ => false,
        }
    }
}

impl RepeatedColors for image::DynamicImage {
    fn get_repeated_colors_rate(&self, number_of_colors: usize) -> f64 {
        use std::collections::HashMap;
        let mut color_count: HashMap<image::Rgb<u8>, f64> = HashMap::new();
        let (width, height) = self.dimensions();

        for (_, _, pixel) in self.pixels() {
            *color_count.entry(pixel.to_rgb()).or_insert(0.0) += 1.0;
        }

        let mut counts: Vec<f64> = color_count.values().copied().collect();
        counts.sort_unstable_by(TotalOrder::total_cmp);

        let top_ten_sum: f64 = counts.iter().take(number_of_colors).sum();
        let total_pixels = f64::from(width) * f64::from(height);
        top_ten_sum / total_pixels
    }
}
