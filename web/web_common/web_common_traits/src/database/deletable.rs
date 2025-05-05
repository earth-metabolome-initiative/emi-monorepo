//! A trait defining a Deletable table entry.

use backend_request_errors::BackendRequestError;

use super::Row;

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

/// Trait for deleting a set of entries from a vector.
pub trait DeleteFromVec {
    /// The type of the rows in the vector.
    type Row: Row;

    /// Updates the content of the vector with the result of the upsert
    /// operation. This operation assumes that the vector is sorted.
    ///
    ///
    /// # Arguments
    ///
    /// * `sorted_rows` - A sorted iterator over the rows to be eliminated from
    ///   the vector.
    ///
    /// # Returns
    ///
    /// The method returns the number of rows that were deleted from the vector.
    ///
    /// # Panics
    ///
    /// * Panics if the vector is not sorted.
    fn delete_from_sorted_vec<I>(&mut self, sorted_rows: I) -> usize
    where
        I: IntoIterator<Item = Self::Row>;
}

impl<T> DeleteFromVec for Vec<T>
where
    T: Row + PartialEq,
{
    type Row = T;

    fn delete_from_sorted_vec<I>(&mut self, sorted_rows: I) -> usize
    where
        I: IntoIterator<Item = Self::Row>,
    {
        let mut new_vector_length = 0;

        for sorted_row in sorted_rows {
            let sorted_row_primary_key = sorted_row.primary_key();
            while self[new_vector_length].primary_key() < sorted_row_primary_key {
                new_vector_length += 1;
            }
        }

        let deleted = self.len() - new_vector_length;
        self.truncate(new_vector_length);

        deleted
    }
}
