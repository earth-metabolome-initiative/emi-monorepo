//! A trait defining a `Loadable` table entry.

/// The `Loadable` trait
pub trait Loadable: Sized {
    /// The primary key type of the table.
    type PrimaryKey;

    #[cfg(feature = "backend")]
    /// Loads the row in a table.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn load(
        primary_key: &Self::PrimaryKey,
        conn: &mut crate::prelude::DBConn,
    ) -> impl std::future::Future<Output = Result<Option<Self>, diesel::result::Error>>;

    #[cfg(feature = "backend")]
    /// Loads all row in a table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading of the row fails.
    fn load_all(
        conn: &mut crate::prelude::DBConn,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, diesel::result::Error>>;
}
