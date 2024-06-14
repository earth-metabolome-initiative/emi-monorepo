use super::*;
use base64::engine::general_purpose;
use base64::Engine;
use image::GenericImageView;
use web_common::api::JPEGError;
use web_common::traits::*;
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
            Err(_) => return Err(JPEGError::InvalidImage.into()),
        };

        // If the image is smaller than 64x64, we reject it
        let (width, height) = image.dimensions();
        const SIZE_LIMIT: u32 = 64;
        if width < SIZE_LIMIT || height < SIZE_LIMIT {
            return Err(JPEGError::ImageTooSmall.into());
        }

        // If the image has transparency, we reject it as we expect JPEGs
        if image.is_transparent() {
            return Err(JPEGError::ImageHasTransparency.into());
        }

        // If the image is larger than 1024 in either width or height, we resize it
        let image = if width > 1024 || height > 1024 {
            image.resize(1024, 1024, image::imageops::FilterType::Lanczos3)
        } else {
            image
        };

        let grayscale = image.to_luma8();

        // If the image is too dark, we reject it
        if grayscale.is_dark(None) {
            return Err(JPEGError::ImageTooDark.into());
        }

        // If the image is too light, we reject it
        if grayscale.is_light(None) {
            return Err(JPEGError::ImageTooLight.into());
        }

        // If the image is blurry, we reject it
        if grayscale.is_blurry(None) {
            return Err(JPEGError::ImageIsBlurry.into());
        }

        // If the image has only a few colors that are repeated a lot, we reject it
        if image.get_repeated_colors_rate(10) > 0.3 {
            return Err(JPEGError::ImageHasFewColors.into());
        }

        // We convert the image to a JPEG
        let mut jpeg = Vec::new();
        let mut jpeg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 80);
        match jpeg_encoder.encode_image(&image) {
            Ok(_) => (),
            Err(_) => return Err(JPEGError::UnableToEncode.into()),
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