//! High-level errors that may occour in the Server.
//!
//! Primarily meant as a way to communicate errors to the client,
//! without exposing internal details.

/// High-level errors that may occour in the Server.
pub enum Error {
    /// An error that occured while trying to access the database.
    DatabaseError,
}

#[cfg(feature = "backend")]
impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error::DatabaseError
    }
}
