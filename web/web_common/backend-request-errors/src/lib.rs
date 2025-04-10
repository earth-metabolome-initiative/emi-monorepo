#![doc = include_str!("../README.md")]

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
/// High-level errors that may occur in the Server.
pub enum BackendRequestError {
    /// An error that occurred on the server.
    InternalServerError,
    /// When something was not found.
    NotFound,
    /// The user attempted an unauthorized action.
    Unauthorized,
}

impl core::fmt::Display for BackendRequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BackendRequestError::InternalServerError => write!(f, "Internal Server error"),
            BackendRequestError::NotFound => write!(f, "Not Found"),
            BackendRequestError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}
