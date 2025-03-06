//! A trait defining a Deletable table entry.

/// The Deletable trait
pub trait Deletable {
    /// The backend to be used.
    type Conn: diesel_async::AsyncConnection;

    /// Deletes the row in a table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the row cannot be deleted.
    fn delete(
        &self,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<usize, diesel::result::Error>>;
}
