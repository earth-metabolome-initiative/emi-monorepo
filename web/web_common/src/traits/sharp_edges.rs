//! Submodule providing the `SharpEdges` trait.

pub trait SharpEdges {
    /// Determines if an image has sharp edges by applying the Canny edge detector
    /// and analyzing the result.
    ///
    /// Parameters:
    /// * `image`: reference to a grayscale image.
    /// * `low_threshold`: lower threshold for the Canny edge detection.
    /// * `high_threshold`: higher threshold for the Canny edge detection.
    ///
    /// Returns:
    /// * A boolean indicating if the image is considered to have sharp edges.
    fn has_sharp_edges(&self, low_threshold: Option<f32>, high_threshold: Option<f32>) -> bool;
}

impl SharpEdges for image::GrayImage {
    fn has_sharp_edges(&self, low_threshold: Option<f32>, high_threshold: Option<f32>) -> bool {
        let low_threshold = low_threshold.unwrap_or(50.0);
        let high_threshold = high_threshold.unwrap_or(100.0);

        // Apply the Canny edge detector to find edges
        let edges = imageproc::edges::canny(self, low_threshold, high_threshold);

        // Analyze the density of edges in the image
        let edge_count: usize = edges.pixels().filter(|&p| p[0] == 255).count();
        let total_pixels = (self.width() * self.height()) as usize;

        // Calculate the percentage of pixels that are considered edges
        let edge_density = edge_count as f32 / total_pixels as f32;

        // Define a threshold for determining "sharpness"
        let edge_density_threshold = 0.05; // Adjust this threshold based on your specific requirements

        edge_density > edge_density_threshold
    }
}
