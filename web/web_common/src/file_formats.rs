//! Submodule handling the complexity of recognizing and parsing the various file formats that the application supports.

use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenericFileFormat {
    JPEG,
    PDF,
}

impl GenericFileFormat {
    pub fn try_from_mime(mime: &str) -> Result<GenericFileFormat, ApiError> {
        match mime {
            // "image/jpeg" | "image/png" | "image/gif" | "image/bmp" => Ok(GenericFileFormat::JPEG),
            "image/jpeg" => Ok(GenericFileFormat::JPEG),
            "application/pdf" => Ok(GenericFileFormat::PDF),
            mime => Err(ApiError::invalid_file_format(format!(
                "The file format {} is not supported.",
                mime
            ))),
        }
    }

    pub fn mime_types(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["image/jpeg"],
            GenericFileFormat::PDF => &["application/pdf"],
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["jpg", "jpeg"],
            GenericFileFormat::PDF => &["pdf"],
        }
    }
}
