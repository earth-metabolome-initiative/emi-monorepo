//! Submodule for the `IsTransparent` trait.
use image::GenericImageView;

pub trait IsTransparent {
    /// Returns whether the provided image is transparent.
    fn is_transparent(&self) -> bool;
}

impl IsTransparent for image::DynamicImage {
    fn is_transparent(&self) -> bool {
        use image::ColorType::*;
        match self.color() {
            La8 | La16 | Rgba8 | Rgba16 => {
                self.pixels().any(|(_, _, pixel)| pixel.0[3] < 220)
            }
            _ => false,
        }
    }
}
