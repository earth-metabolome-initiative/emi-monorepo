//! Submodule providing structs to validate Images, provided as a vector of bytes.
//!
//! # Implementation details

use image::io::Reader;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Formatter;
use validator::Validate;
use validator::ValidationError;

use crate::api::ApiError;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Image {
    data: Vec<u8>,
}

impl Into<Vec<u8>> for Image {
    fn into(self) -> Vec<u8> {
        self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "standard")]
    Standard,
}

pub const STANDARD_IMAGE_SIZE: u32 = 512;

impl ImageSize {
    pub fn width(&self) -> u32 {
        match self {
            ImageSize::Thumbnail => 64,
            ImageSize::Standard => STANDARD_IMAGE_SIZE,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            ImageSize::Thumbnail => 64,
            ImageSize::Standard => STANDARD_IMAGE_SIZE,
        }
    }
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ImageSize::Thumbnail => write!(f, "thumbnail"),
            ImageSize::Standard => write!(f, "standard"),
        }
    }
}

impl AsRef<Image> for Image {
    fn as_ref(&self) -> &Image {
        self
    }
}

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub trait TryFromImage: Sized {
    fn try_from_image(image: Image) -> Result<Self, ApiError>;
}

impl Validate for Image {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match Image::is_image(&self.data) {
            Some(_) => Ok(()),
            None => {
                let mut errors = validator::ValidationErrors::new();
                errors.add("data", ValidationError::new("Data is not a valid image."));
                Err(errors)
            }
        }
    }
}

#[cfg(feature = "frontend")]
impl crate::api::form_traits::TryFromCallback<web_sys::File> for Image {
    fn try_from_callback<C>(file: web_sys::File, callback: C) -> Result<(), ApiError>
    where
        C: FnOnce(Result<Self, ApiError>) + 'static,
    {
        use wasm_bindgen::JsCast;

        // Create a reader
        let reader = web_sys::FileReader::new()
            .map_err(|_| vec!["Unable to open File Reader".to_string()])?;

        let on_load =
            wasm_bindgen::closure::Closure::once_into_js(move |event: web_sys::ProgressEvent| {
                let reader = event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::FileReader>()
                    .unwrap();

                let result = reader.result().unwrap();
                let data = js_sys::Uint8Array::new(&result);
                let data = data.to_vec();

                match Image::try_from(data) {
                    Ok(image) => callback(Ok(image)),
                    Err(errors) => callback(Err(ApiError::BadRequest(errors))),
                };
            });

        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));

        // Read file contents
        reader
            .read_as_array_buffer(&file)
            .map_err(|_| vec!["Unable to read file.".to_string()])?;

        Ok(())
    }
}

impl TryFrom<Vec<u8>> for Image {
    type Error = Vec<String>;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(vec!["Image data is empty.".to_string()]);
        }

        match Image::is_image(&data) {
            Some(format) => match format {
                ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::Gif | ImageFormat::WebP => {
                    Ok(Image { data })
                }
                _ => Err(vec![format!("Image format {:?} is not supported.", format)]),
            },
            None => Err(vec!["Data is not a valid image.".to_string()]),
        }
    }
}

impl TryFromImage for Image {
    fn try_from_image(image: Image) -> Result<Self, ApiError> {
        Ok(image)
    }
}

impl Image {
    fn is_image(bytes: &[u8]) -> Option<ImageFormat> {
        Reader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.format())
    }

    #[cfg(feature = "backend")]
    /// Cuts a square from the image, centered around the face detected in the image.
    ///
    /// # Implementative details
    /// The face detection is implemented using the `rustface` crate.
    pub fn to_face_square(&self) -> Result<DynamicImage, Vec<String>> {
        let faces = crate::custom_validators::images::contains_face::get_faces(self)
            .map_err(|e| vec![e.to_string()])?;

        if faces.len() != 1 {
            return Err(vec!["Expected exactly one face.".to_string()]);
        }

        let face = faces[0].bbox();

        // We get the central coordinates of the face
        let mut x = face.x() as u32 + face.width() / 2;
        let mut y = face.y() as u32 + face.height() / 2;

        let mut image = image::load_from_memory(&self.data).map_err(|e| vec![e.to_string()])?;
        let (width, height) = image.dimensions();
        let size = std::cmp::min(width, height);

        // We need to crop the image to a square
        let mut square = image::DynamicImage::new_rgba8(size, size);

        x = x.saturating_sub(size);
        y = y.saturating_sub(size);
        let cropped_image = image.crop(x, y, x + size, y + size);

        // Paste the cropped image onto the square canvas
        square
            .copy_from(&cropped_image, 0, 0)
            .map_err(|e| vec![e.to_string()])?;

        Ok(square)
    }

    pub fn shape(&self) -> Result<(u32, u32), String> {
        let reader = Reader::new(std::io::Cursor::new(&self.data))
            .with_guessed_format()
            .map_err(|e| e.to_string())?;
        let dimensions = reader.into_dimensions().map_err(|e| e.to_string())?;
        Ok((dimensions.0, dimensions.1))
    }
}
