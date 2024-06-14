//! Utiliti traits for formatting and matching strings.

use base64::engine::general_purpose;
use base64::Engine;
use image::ImageFormat;
mod is_blurry;
pub use is_blurry::IsBlurry;
mod is_transparent;
pub use is_transparent::IsTransparent;
mod repeated_colors;
pub use repeated_colors::RepeatedColors;
mod sharp_edges;
pub use sharp_edges::SharpEdges;
mod is_dark;
pub use is_dark::IsDark;
mod is_light;
pub use is_light::IsLight;


pub trait CapitalizeString {
    /// Returns the provided string with the first letter capitalized.
    fn capitalize(&self) -> String;
}

impl<S: AsRef<str>> CapitalizeString for S {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

pub trait GuessImageFormat: AsRef<[u8]> {
    /// Returns the image format based on the provided bytes.
    fn guess_image_format(&self) -> Option<ImageFormat> {
        image::io::Reader::new(std::io::Cursor::new(self.as_ref()))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.format())
    }

    /// Returns the image url based on the provided bytes.
    fn guess_image_url(&self) -> Option<String> {
        self.guess_image_format().map(|f| {
            format!(
                "data:image/{};base64,{}",
                f.extensions_str()[0],
                general_purpose::STANDARD.encode(self.as_ref())
            )
        })
    }
}

impl<T: AsRef<[u8]>> GuessImageFormat for T {}

pub trait TryIntoBytea {
    /// Returns the provided object as a byte array.
    fn try_into_bytea(&self) -> Result<Vec<u8>, crate::api::ApiError>;
}

impl TryIntoBytea for image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
    fn try_into_bytea(&self) -> Result<Vec<u8>, crate::api::ApiError> {
        // By default, the image is saved as a JPEG with a quality of 75.
        let mut buf = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75);
        encoder.encode(
            self.as_ref(),
            self.width(),
            self.height(),
            image::ColorType::Rgba8.into(),
        )?;
        Ok(buf)
    }
}
