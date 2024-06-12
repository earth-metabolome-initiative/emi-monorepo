use super::*;
use base64::engine::general_purpose;
use base64::Engine;
use image::GenericImageView;
use web_common::types::JPEG;

impl FileLike for JPEG {
    const FORMATS: &'static [GenericFileFormat] =
        &[GenericFileFormat::JPEG, GenericFileFormat::PNG];

    fn file_size(&self) -> u64 {
        let data: &[u8] = self.as_ref();
        data.len() as u64
    }

    async fn try_from_bytes(data: Vec<u8>) -> Result<Self, ApiError> {
        // We try to load the image to ensure it is a valid JPEG
        let image = match image::load_from_memory(&data) {
            Ok(image) => image,
            Err(_) => return Err(ApiError::from(vec!["Invalid JPEG file.".to_string()])),
        };

        // If the image is larger than 1024 in either width or height, we resize it
        let (width, height) = image.dimensions();
        let image = if width > 1024 || height > 1024 {
            image.resize(1024, 1024, image::imageops::FilterType::Lanczos3)
        } else {
            image
        };

        // We convert the image to a JPEG
        let mut jpeg = Vec::new();
        let mut jpeg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 80);
        match jpeg_encoder.encode_image(&image) {
            Ok(_) => (),
            Err(_) => return Err(ApiError::from(vec!["Unable to encode JPEG.".to_string()])),
        }

        Ok(jpeg.into())
    }

    fn preview(&self) -> Html {
        let data: &[u8] = self.as_ref();

        let url = format!(
            "data:image/jpeg;base64,{}",
            general_purpose::STANDARD.encode(data)
        );
        html! {
            <img src={url} class="preview" />
        }
    }
}
