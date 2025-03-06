//! A trait defining a `Loadable` table entry.

/// The `Loadable` trait
pub trait Loadable: Sized + diesel::associations::HasTable {
    /// The primary key type of the table.
    type PrimaryKey;

    /// The connection type of the table.
    type Conn: diesel_async::AsyncConnection;

    /// Loads the row in a table.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to a
    ///   [`diesel::connection::LoadConnection`](diesel::connection::LoadConnection).
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn load(
        primary_key: &Self::PrimaryKey,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<Option<Self>, diesel::result::Error>>;

    /// Loads all row in a table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a
    ///   [`diesel::connection::LoadConnection`](diesel::connection::LoadConnection).
    ///
    /// # Errors
    ///
    /// * Returns an error if loading of the row fails.
    fn load_all(
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<Vec<Self>, diesel::result::Error>>;
}
