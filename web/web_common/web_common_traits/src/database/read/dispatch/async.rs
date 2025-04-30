#![cfg(feature = "diesel-async")]
//! Asynchronous dispatch for the `Read` and `BoundedRead` traits.
use diesel_async::AsyncConnection;

/// Variant of the `AsyncRead` trait meant to be implemented by enumeration of
/// types.
pub trait AsyncReadDispatch<C>: Sized {
    /// The type of the primary key of the table.
    type PrimaryKey;

    /// Returns the entry associated with the provided primary key.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to an asynchronous connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn read(
        primary_key: Self::PrimaryKey,
        conn: &mut C,
    ) -> impl Future<Output = Result<Option<Self>, diesel::result::Error>>;
}

/// Variant of the `BoundedRead` trait meant to be implemented by enumeration of
/// types.
pub trait AsyncBoundedReadDispatch<C: AsyncConnection>: Sized {
    /// The type of the table name.
    type TableName;

    /// Returns the rows in the provided range.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to be loaded.
    /// * `offset` - The offset of the first row to be loaded.
    /// * `limit` - The maximum number of rows to be loaded.
    /// * `conn` - A mutable reference to an asynchronous connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn bounded_read(
        table_name: Self::TableName,
        offset: u64,
        limit: u64,
        conn: &mut C,
    ) -> impl Future<Output = Result<Self, diesel::result::Error>>;
}
