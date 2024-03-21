//! Submodule providing a validator that requires that the provided image does not contain nudity.
//!
//! # Implementation details
//! This implementation is based on the `nude` crate. Since this crate requires several
//! non-WASM compatible dependencies, this validator is only available in the backend and we
//! transparently provide a dummy implementation in the frontend. We acknowledge that this
//! implementation is not perfect and that it might not catch all nudity and might also
//! produce false positives. We also acknowledge that this is a sensitive topic and that
//! this implementation might not be suitable for all use cases.

#[cfg(feature = "backend")]
mod backend {
    use crate::custom_validators::images::image::Image;
    use validator::ValidationError;
    use web_common_derive::image_validator;

    #[image_validator("We expected the image to not contain nudity.")]
    pub fn no_nudes<I>(data: &I) -> Result<(), ValidationError>
    where
        I: AsRef<Image>,
    {
        log::info!("Scanning image for nudity.");
        let img = image::load_from_memory(data.as_ref().as_ref())
            .map_err(|_| ValidationError::new("Invalid image data."))?;
        let nudity = nude::scan(&img).analyse();
        if nudity.score() > 1.1 {
            return Err(ValidationError::new("Nudity detected."));
        }
        Ok(())
    }
}

#[cfg(feature = "frontend")]
mod frontend {
    use crate::custom_validators::images::image::Image;
    use validator::ValidationError;
    use web_common_derive::image_validator;

    #[image_validator("We expected the image to not contain nudity.")]
    pub fn no_nudes<I>(_data: &I) -> Result<(), ValidationError>
    where
        I: AsRef<Image>,
    {
        Ok(())
    }
}

#[cfg(feature = "backend")]
pub use backend::*;

#[cfg(not(feature = "backend"))]
pub use frontend::*;
