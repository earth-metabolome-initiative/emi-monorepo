//! Submodule handling the complexity of recognizing and parsing the various
//! file formats that the application supports.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenericFileFormat {
    JPEG,
    PNG,
    PDF,
}

impl GenericFileFormat {
    pub fn mime_types(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["image/jpeg"],
            GenericFileFormat::PNG => &["image/png"],
            GenericFileFormat::PDF => &["application/pdf"],
        }
    }

    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["jpg", "jpeg"],
            GenericFileFormat::PNG => &["png"],
            GenericFileFormat::PDF => &["pdf"],
        }
    }
}
