//! Crate providing common validation errors.

#[derive(Debug, Clone, PartialEq)]
/// Enumeration of errors that can occur during validation.
pub enum Error {
    /// The provided text is empty.
    EmptyText,
    /// The provided mail address is invalid.
    InvalidMail,
    /// The float is not strictly positive (0.0, ...]
    UnexpectedNegativeOrZeroValue,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EmptyText => write!(f, "The provided text is empty."),
            Error::InvalidMail => write!(f, "The provided mail address is invalid."),
            Error::UnexpectedNegativeOrZeroValue => {
                write!(f, "The provided value must be strictly positive and none zero.")
            }
        }
    }
}
