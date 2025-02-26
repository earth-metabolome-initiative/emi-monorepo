//! A trait defining a deletable table entry.

/// The Deletable trait
pub trait Deletable {
    #[cfg(feature = "backend")]
    /// Deletes the row in a table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the row cannot be deleted.
    ///
    fn delete(
        &self,
        conn: &mut crate::prelude::DBConn,
    ) -> impl std::future::Future<Output = Result<usize, diesel::result::Error>> + '_;
}
