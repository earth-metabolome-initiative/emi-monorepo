//! Errors that may occur when using Font Awesome icons.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumeration of errors that may occur when using Font Awesome icons.
pub enum Error {
    /// A provided class was not found.
    UnknownClass(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::UnknownClass(class) => write!(f, "Unknown class: {class}"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
