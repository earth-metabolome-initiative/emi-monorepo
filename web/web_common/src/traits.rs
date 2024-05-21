//! Utiliti traits for formatting and matching strings.

use image::ImageFormat;
use base64::engine::general_purpose;
use base64::Engine;

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
