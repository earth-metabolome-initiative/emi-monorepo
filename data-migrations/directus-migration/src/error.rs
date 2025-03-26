//! Enumeration of errors that may occur during directus migration

#[derive(Debug)]
/// Enumeration of errors that may occur during directus migration
pub enum Error{
    /// Error when user doesn't have an email
    MissingEmail(uuid::Uuid),
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
}

impl From<diesel::ConnectionError> for Error{
    fn from(value: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(value)
    }
}

impl From<diesel::result::Error> for Error{
    fn from(value: diesel::result::Error) -> Self {
        Error::QueryFailed(value)
    }
}