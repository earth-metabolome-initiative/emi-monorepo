//! Submodule providing an error enumeration for traits defined within the
//! `image-validation` crate.

#[derive(Debug, Clone, Copy)]
/// Enumeration of errors that can occur in the image validation library.
pub enum Error {
    /// Error indicating that the threshold value is out of range.
    /// The threshold must be between 0.0 and 1.0.
    ThresholdOutOfRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ThresholdOutOfRange => {
                write!(f, "Threshold value is out of range. Must be between 0.0 and 1.0.")
            }
        }
    }
}

impl std::error::Error for Error {}
