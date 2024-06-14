//! Type representing a JPEG image as bytes.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// Type representing a JPEG image as bytes.
pub struct JPEG(Vec<u8>);

/// Implement Default for JPEG, which generates a 128x128 black JPEG image
/// using the image crate.
impl Default for JPEG {
    fn default() -> Self {
        use image::{DynamicImage, ImageBuffer, Rgba};
        let img = ImageBuffer::from_fn(128, 128, |_, _| Rgba([0, 0, 0, 255]));
        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, 50);
        encoder
            .encode(
                &DynamicImage::ImageRgba8(img).into_bytes(),
                128,
                128,
                image::ColorType::Rgba8.into(),
            )
            .unwrap();
        JPEG(cursor.into_inner())
    }
}

impl AsRef<[u8]> for JPEG {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<Vec<u8>> for JPEG {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl AsRef<JPEG> for JPEG {
    fn as_ref(&self) -> &JPEG {
        self
    }
}

impl From<JPEG> for Vec<u8> {
    fn from(jpeg: JPEG) -> Vec<u8> {
        jpeg.0
    }
}

impl From<Vec<u8>> for JPEG {
    fn from(bytes: Vec<u8>) -> Self {
        JPEG(bytes)
    }
}
