//! Crate providing common validation errors.

#[derive(Debug, Clone)]
/// Enumeration of errors that can occur during validation.
pub enum Error {
    /// The provided text is empty.
    EmptyText,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EmptyText => write!(f, "The provided text is empty."),
        }
    }
}
