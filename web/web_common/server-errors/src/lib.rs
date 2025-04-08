//! High-level errors that may occour in the Server.
//!
//! Primarily meant as a way to communicate errors to the client,
//! without exposing internal details.

/// High-level errors that may occour in the Server.
pub enum Error {
    /// An error that occured while trying to access the database.
    DatabaseError,
    /// The user attempted an unauthorized action.
    Unauthorized,
}

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error::DatabaseError
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::DatabaseError => {
                write!(f, "An error occured while trying to access the database.")
            }
            Error::Unauthorized => write!(f, "The user attempted an unauthorized action."),
        }
    }
}
