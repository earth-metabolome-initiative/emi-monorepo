//! Submodule defining a trait for types that can be updated in the database.

/// A trait for types that can be updated in the database.
pub trait Updatable<C> {
    /// The expected user ID type.
    type UserId;

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
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error>;
}
