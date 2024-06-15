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

impl From<Vec<u8>> for Image {
    fn from(value: Vec<u8>) -> Self {
        Self { data: value }
    }
}

impl From<&Vec<u8>> for Image {
    fn from(data: &Vec<u8>) -> Self {
        Self {
            data: data.clone(),
        }
    }
}

impl From<&[u8]> for Image {
    fn from(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

impl From<Image> for Vec<u8> {
    fn from(image: Image) -> Vec<u8> {
        image.data
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


// impl TryFrom<Vec<u8>> for Image {
//     type Error = Vec<String>;

//     fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
//         if data.is_empty() {
//             return Err(vec!["Image data is empty.".to_string()]);
//         }

//         match Image::is_image(&data) {
//             Some(format) => match format {
//                 ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::Gif | ImageFormat::WebP => {
//                     Ok(Image { data })
//                 }
//                 _ => Err(vec![format!("Image format {:?} is not supported.", format)]),
//             },
//             None => Err(vec!["Data is not a valid image.".to_string()]),
//         }
//     }
// }

impl TryFromImage for Image {
    fn try_from_image(image: Image) -> Result<Self, ApiError> {
        Ok(image)
    }
}

impl Image {
    pub fn is_image(bytes: &[u8]) -> Option<ImageFormat> {
        Reader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.format())
    }

    pub fn shape(&self) -> Result<(u32, u32), String> {
        let reader = Reader::new(std::io::Cursor::new(&self.data))
            .with_guessed_format()
            .map_err(|e| e.to_string())?;
        let dimensions = reader.into_dimensions().map_err(|e| e.to_string())?;
        Ok((dimensions.0, dimensions.1))
    }
}
