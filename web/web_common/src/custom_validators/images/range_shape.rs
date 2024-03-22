//! Submodule providing the structs RangeShape, ExactShape, MinShape and MaxShape, which
//! are used to validate the shape of an image. Since these structs need to be used as a type
//! including the shape parameters, the ranges are defined as constants of the structs. The most
//! general of the structs is RangeShape, which is used to validate that the width and height of
//! the image are within the specified ranges. The other structs are variants of the RangeShape
//! with one of the ranges fixed to a specific value, so as to minimize the code duplication.

use crate::custom_validators::validation_errors::ValidationErrorToString;
use crate::custom_validators::Image;
use crate::custom_validators::TryFromImage;
use std::borrow::Cow;
use validator::Validate;
use validator::ValidationError;

pub fn validate_range<
    V,
    const MIN_WIDTH: u32,
    const MAX_WIDTH: u32,
    const MIN_HEIGHT: u32,
    const MAX_HEIGHT: u32,
>(
    shape: &RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>,
) -> Result<(), ValidationError>
where
    V: AsRef<Image> + serde::Serialize + validator::Validate,
{
    let (width, height) = shape
        .as_ref()
        .shape()
        .map_err(|_| ValidationError::new("invalid_range_shape"))?;

    if width < MIN_WIDTH && height < MIN_HEIGHT {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "Both the image width and height are less than the expected minimum.",
        ));
        return Err(validation_error);
    }

    if width > MAX_WIDTH && height > MAX_HEIGHT {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "Both the image width and height are greater than the expected maximum.",
        ));
        return Err(validation_error);
    }

    if width < MIN_WIDTH {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "The image width is less than the expected minimum.",
        ));
        return Err(validation_error);
    }

    if width > MAX_WIDTH {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "The image shape is not within the specified range.",
        ));
        return Err(validation_error);
    }

    if height < MIN_HEIGHT {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "The image height is less than the expected minimum.",
        ));
        return Err(validation_error);
    }

    if height > MAX_HEIGHT {
        let mut validation_error = ValidationError::new("invalid_range_shape");
        validation_error.message = Some(Cow::from(
            "The image shape is not within the specified range.",
        ));
        return Err(validation_error);
    }

    Ok(())
}

#[repr(transparent)]
#[derive(
    Debug,
    validator::Validate,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    Clone,
    Default,
    serde::Deserialize,
)]
#[validate(schema(function = "validate_range", skip_on_field_errors = true,))]
pub struct RangeShape<
    V,
    const MIN_WIDTH: u32 = 0,
    const MAX_WIDTH: u32 = { u32::MAX },
    const MIN_HEIGHT: u32 = 0,
    const MAX_HEIGHT: u32 = { u32::MAX },
> where
    V: AsRef<Image> + serde::Serialize + validator::Validate,
{
    #[validate]
    value: V,
}

impl<
        V,
        const MIN_WIDTH: u32,
        const MAX_WIDTH: u32,
        const MIN_HEIGHT: u32,
        const MAX_HEIGHT: u32,
    > AsRef<Image> for RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
where
    V: AsRef<Image> + serde::Serialize + validator::Validate,
{
    fn as_ref(&self) -> &Image {
        self.value.as_ref()
    }
}

#[cfg(feature = "frontend")]
impl<
        V,
        const MIN_WIDTH: u32,
        const MAX_WIDTH: u32,
        const MIN_HEIGHT: u32,
        const MAX_HEIGHT: u32,
    > crate::api::form_traits::TryFromCallback<web_sys::File>
    for RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
where
    V: crate::api::form_traits::TryFromCallback<web_sys::File>
        + AsRef<Image>
        + serde::Serialize
        + validator::Validate,
{
    fn try_from_callback<C>(file: web_sys::File, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Self, Vec<String>>) + 'static,
    {
        V::try_from_callback(file, move |value| {
            match value {
                Err(e) => callback(Err(e)),
                Ok(value) => {
                    let maybe_self = Self { value };
                    match maybe_self.validate() {
                        Ok(()) => callback(Ok(maybe_self)),
                        Err(e) => callback(Err(e.convert_to_string())),
                    };
                }
            };
        })
    }
}

impl<
        V,
        const MIN_WIDTH: u32,
        const MAX_WIDTH: u32,
        const MIN_HEIGHT: u32,
        const MAX_HEIGHT: u32,
    > TryFromImage for RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
where
    V: crate::custom_validators::image::TryFromImage
        + AsRef<Image>
        + serde::Serialize
        + validator::Validate,
{
    fn try_from_image(image: Image) -> Result<Self, Vec<String>> {
        let maybe_self = Self {
            value: V::try_from_image(image)?,
        };
        if let Err(e) = maybe_self.validate() {
            return Err(e.convert_to_string());
        }
        Ok(maybe_self)
    }
}

impl<
        V,
        const MIN_WIDTH: u32,
        const MAX_WIDTH: u32,
        const MIN_HEIGHT: u32,
        const MAX_HEIGHT: u32,
    > TryFrom<Image> for RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
where
    V: crate::custom_validators::image::TryFromImage
        + AsRef<Image>
        + serde::Serialize
        + validator::Validate,
{
    type Error = Vec<String>;

    fn try_from(value: Image) -> Result<Self, Self::Error> {
        let maybe_self = Self {
            value: V::try_from_image(value)?,
        };
        if let Err(e) = maybe_self.validate() {
            return Err(e.convert_to_string());
        }
        Ok(maybe_self)
    }
}

impl<
        V,
        const MIN_WIDTH: u32,
        const MAX_WIDTH: u32,
        const MIN_HEIGHT: u32,
        const MAX_HEIGHT: u32,
    > From<RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>> for Image
where
    V: AsRef<Image> + serde::Serialize + validator::Validate + Into<Image>,
{
    fn from(value: RangeShape<V, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>) -> Self {
        value.value.into()
    }
}

pub type ExactShape<V, const WIDTH: u32, const HEIGHT: u32> =
    RangeShape<V, WIDTH, WIDTH, HEIGHT, HEIGHT>;
pub type MinShape<V, const MIN_WIDTH: u32, const MIN_HEIGHT: u32> =
    RangeShape<V, MIN_WIDTH, { u32::MAX }, MIN_HEIGHT, { u32::MAX }>;
pub type MaxShape<V, const MAX_WIDTH: u32, const MAX_HEIGHT: u32> =
    RangeShape<V, 0, MAX_WIDTH, 0, MAX_HEIGHT>;
