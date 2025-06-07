#![doc = include_str!("../README.md")]

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
/// High-level errors that may occur in the Server.
pub enum GenericBackendRequestError {
    /// An error that occurred on the server.
    InternalServerError,
    /// When something was not found.
    NotFound,
    /// The user attempted an unauthorized action.
    Unauthorized,
}

impl core::fmt::Display for GenericBackendRequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InternalServerError => write!(f, "Internal Server error"),
            Self::NotFound => write!(f, "Not Found"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}
