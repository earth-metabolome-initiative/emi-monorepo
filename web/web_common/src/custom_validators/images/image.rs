//! Submodule providing structs to validate Images, provided as a vector of bytes.
//!
//! # Implementation details

use image::io::Reader;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator::ValidationError;

use crate::api::form_traits::TryFromCallback;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Image {
    data: Vec<u8>,
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
    fn try_from_image(image: Image) -> Result<Self, Vec<String>>;
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
impl TryFromCallback<web_sys::File> for Image {
    fn try_from_callback<C>(file: web_sys::File, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Self, Vec<String>>) + 'static,
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
                    Err(errors) => callback(Err(errors)),
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
    fn try_from_image(image: Image) -> Result<Self, Vec<String>> {
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

    pub fn shape(&self) -> Result<(u32, u32), String> {
        let reader = Reader::new(std::io::Cursor::new(&self.data))
            .with_guessed_format()
            .map_err(|e| e.to_string())?;
        let dimensions = reader.into_dimensions().map_err(|e| e.to_string())?;
        Ok((dimensions.0, dimensions.1))
    }
}
