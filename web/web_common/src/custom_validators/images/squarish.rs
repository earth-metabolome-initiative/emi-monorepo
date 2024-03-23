//! Submodule providing a validator that requires that the provided image is squarish, i.e.
//! that the width and height are no more than 1.5 times different.

use crate::custom_validators::images::image::Image;
use web_common_derive::image_validator;

#[image_validator("We expected the image to be squarish, this one is too rectangular.")]
pub fn squarish<I>(data: &I) -> Result<(), validator::ValidationError>
where
    I: AsRef<Image>,
{
    log::info!("Checking if the image is squarish.");
    let image = data.as_ref();
    let (width, height) = image
        .shape()
        .map_err(|_| validator::ValidationError::new("Invalid image data."))?;
    if (width as f32 / height as f32) > 1.5 || (height as f32 / width as f32) > 1.5 {
        return Err(validator::ValidationError::new(
            "The image is not squarish.",
        ));
    }
    Ok(())
}
