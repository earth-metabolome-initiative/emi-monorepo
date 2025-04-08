//! Submodule defining a trait for types that can be updated in the database.

/// A trait for types that can be updated in the database.
pub trait Updatable {
    /// The expected user ID type.
    type UserId;
    /// The connection type of the table.
    type Conn;

    /// Returns whether the user can update the row.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the user can update the row.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    fn can_update(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<bool, diesel::result::Error>>;
}
