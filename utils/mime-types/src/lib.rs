#![doc = include_str!("../README.md")]

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
/// Enum representing the generic file formats.
pub enum GenericFileFormat {
    /// JPEG image format.
    JPEG,
    /// PNG image format.
    PNG,
    /// PDF document format.
    PDF,
}

impl GenericFileFormat {
    #[must_use]
    /// Returns the MIME types associated with the file format.
    pub fn mime_types(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["image/jpeg"],
            GenericFileFormat::PNG => &["image/png"],
            GenericFileFormat::PDF => &["application/pdf"],
        }
    }

    #[must_use]
    /// Returns the file extensions associated with the file format.
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            GenericFileFormat::JPEG => &["jpg", "jpeg"],
            GenericFileFormat::PNG => &["png"],
            GenericFileFormat::PDF => &["pdf"],
        }
    }
}
