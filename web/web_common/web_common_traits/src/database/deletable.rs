//! A trait defining a Deletable table entry.

use backend_request_errors::BackendRequestError;

/// The Deletable trait
pub trait Deletable {
    /// The backend to be used.
    type Conn;
    /// The expected user ID type.
    type UserId;

    /// Deletes the row in a table.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the row cannot be deleted.
    /// * Returns an error if the user is not authorized to delete the row.
    fn delete(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<bool, DeleteError>>;
}

/// The error type for deletions.
pub enum DeleteError {
    /// A diesel error occurred.
    DieselError(diesel::result::Error),
    /// A server error occurred.
    ServerError(BackendRequestError),
}

impl From<diesel::result::Error> for DeleteError {
    fn from(e: diesel::result::Error) -> Self {
        DeleteError::DieselError(e)
    }
}

impl From<BackendRequestError> for DeleteError {
    fn from(e: BackendRequestError) -> Self {
        DeleteError::ServerError(e)
    }
}

impl core::fmt::Display for DeleteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeleteError::DieselError(e) => <diesel::result::Error as core::fmt::Display>::fmt(e, f),
            DeleteError::ServerError(e) => <BackendRequestError as core::fmt::Display>::fmt(e, f),
        }
    }
}
