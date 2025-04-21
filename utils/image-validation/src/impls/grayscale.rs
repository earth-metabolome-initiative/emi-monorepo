//! Implementation of traits for [`GrayImage`](image::GrayImage).

use rustfft::{FftPlanner, num_complex::Complex};

use crate::traits::{is_blurry::IsBlurry, is_dark::IsDark, is_light::IsLight};

impl IsBlurry for image::GrayImage {
    fn is_blurry(&self, sharpness_threshold: Option<f32>) -> bool {
        // Perform FFT on the original image (no added blur)
        let fft_data = compute_fft(self);
        let mean_high_freq = analyze_high_freq(&fft_data);

        // Define a threshold below which the image is considered blurry.
        // This threshold would need to be calibrated with real-world data to be
        // effective.
        let sharpness_threshold = sharpness_threshold.unwrap_or(5000.0);

        mean_high_freq < sharpness_threshold
    }
}

fn compute_fft(image: &image::GrayImage) -> Vec<Complex<f32>> {
    let (width, height) = image.dimensions();
    let mut input: Vec<Complex<f32>> =
        image.pixels().map(|p| Complex::new(f32::from(p[0]), 0.0)).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(width as usize * height as usize);
    fft.process(&mut input);
    input
}

fn analyze_high_freq(data: &[Complex<f32>]) -> f32 {
    let length = data.len();
    let window_size = (length as f32 / 2.1).round() as usize;
    let center_index = length / 2;
    let high_freq_values: Vec<f32> = data.iter().map(|x| x.norm()).collect();

    let window = &high_freq_values[center_index - window_size..center_index + window_size];
    window.iter().sum::<f32>() / window.len() as f32
}

impl IsDark for image::GrayImage {
    #[allow(clippy::cast_precision_loss)]
    fn is_dark(&self, threshold: Option<f32>) -> bool {
        let threshold = threshold.unwrap_or(0.1);
        let threshold_u8 = (threshold * 255.0) as u8;
        let mut dark_pixels = 0;
        for pixel in self.pixels() {
            if pixel.0[0] < threshold_u8 {
                dark_pixels += 1;
            }
        }
        dark_pixels > self.width() * self.height() / 2
    }
}

impl IsLight for image::GrayImage {
    #[allow(clippy::cast_precision_loss)]
    fn is_light(&self, threshold: Option<f32>) -> bool {
        let threshold = threshold.unwrap_or(0.8);
        let threshold_u8 = (threshold * 255.0) as u8;
        let mut light_pixels = 0;
        for pixel in self.pixels() {
            if pixel.0[0] > threshold_u8 {
                light_pixels += 1;
            }
        }
        light_pixels > self.width() * self.height() / 2
    }
}
