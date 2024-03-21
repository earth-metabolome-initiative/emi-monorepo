//! Submodule providing a validator that checks if an image contains one or more faces.
//!
//! # Implementation details
//! This implementation is based on the `rustface` crate. Since this crate requires several
//! non-WASM compatible dependencies, this validator is only available in the backend and we
//! transparently provide a dummy implementation in the frontend.

#[cfg(feature = "backend")]
mod backend {
    use crate::custom_validators::images::image::Image;
    use rustface::ImageData;
    use validator::ValidationError;
    use web_common_derive::image_validator;

    pub fn get_number_of_faces<I>(data: &I) -> Result<usize, ValidationError>
    where
        I: AsRef<Image>,
    {
        log::info!("Loading face recognition model.");
        let mut detector = rustface::create_detector("/app/face_recognition.bin").unwrap();
        detector.set_min_face_size(20);
        detector.set_score_thresh(2.0);
        detector.set_pyramid_scale_factor(0.8);
        detector.set_slide_window_step(4, 4);

        let image = image::load_from_memory(data.as_ref().as_ref())
            .map_err(|_| ValidationError::new("Invalid image data."))?;

        let gray = image.to_luma8();

        let (width, height) = gray.dimensions();
        let mut image = ImageData::new(&gray, width, height);
        let faces = detector.detect(&mut image);
        Ok(faces.len())
    }

    #[image_validator("We expected the image to contain a human face.")]
    pub fn contains_face<I>(data: &I) -> Result<(), ValidationError>
    where
        I: AsRef<Image>,
    {
        if get_number_of_faces(data)? == 0 {
            return Err(ValidationError::new("No face detected."));
        }
        Ok(())
    }

    #[image_validator("We expected the image to contain exactly one human face.")]
    pub fn contains_one_face<I>(data: &I) -> Result<(), ValidationError>
    where
        I: AsRef<Image>,
    {
        let number_of_faces = get_number_of_faces(data)?;
        log::info!("Number of faces: {}", number_of_faces);
        if number_of_faces != 1 {
            return Err(ValidationError::new("More than one face detected."));
        }
        Ok(())
    }

    #[image_validator("We expected the image to not contain any human faces.")]
    pub fn no_faces<I>(data: &I) -> Result<(), ValidationError>
    where
        I: AsRef<Image>,
    {
        if get_number_of_faces(data)? != 0 {
            return Err(ValidationError::new("A face was detected."));
        }
        Ok(())
    }
}

#[cfg(feature = "frontend")]
mod frontend {
    use crate::custom_validators::images::image::Image;
    use web_common_derive::image_validator;

    #[image_validator("We expected the image to contain a human face.")]
    pub fn contains_face<I>(_data: &I) -> Result<(), validator::ValidationError>
    where
        I: AsRef<Image>,
    {
        log::warn!("The `contains_face` validator is not available in the frontend.");
        Ok(())
    }

    #[image_validator("We expected the image to contain exactly one human face.")]
    pub fn contains_one_face<I>(_data: &I) -> Result<(), validator::ValidationError>
    where
        I: AsRef<Image>,
    {
        log::warn!("The `contains_one_face` validator is not available in the frontend.");
        Ok(())
    }

    #[image_validator("We expected the image to not contain any human faces.")]
    pub fn no_faces<I>(_data: &I) -> Result<(), validator::ValidationError>
    where
        I: AsRef<Image>,
    {
        log::warn!("The `no_faces` validator is not available in the frontend.");
        Ok(())
    }
}

#[cfg(feature = "backend")]
pub use backend::*;

#[cfg(not(feature = "backend"))]
pub use frontend::*;
