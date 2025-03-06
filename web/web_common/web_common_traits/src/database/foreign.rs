//! Traits relative to the presence of foreign keys in a struct.

/// Trait for a struct that has a foreign key.
pub trait Foreign<T> {
    /// The connection type of the table.
    type Conn: diesel_async::AsyncConnection;

    /// Returns the foreign key.
    fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<T, diesel::result::Error>>;
}
