#![doc = include_str!("../README.md")]

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
/// High-level errors that may occur in the Server.
pub enum BackendRequestError {
    /// An error that occurred on the server.
    ServerError,
    /// The user attempted an unauthorized action.
    Unauthorized,
}

impl core::fmt::Display for BackendRequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BackendRequestError::ServerError => write!(f, "Server error"),
            BackendRequestError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}
