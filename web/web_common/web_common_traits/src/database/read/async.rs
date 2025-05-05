#![cfg(feature = "diesel-async")]
//! Submodule providing traits and blanked implementations for the asynchronous
//! `Read` operation.

use diesel::Identifiable;
use diesel_async::AsyncConnection;

mod postgres;

/// Asynchronous variant of the `Read` trait.
pub trait AsyncRead<C: AsyncConnection>: Identifiable + Sized + Send {
    /// Loads the row in a table.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to an asynchronous connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn read_async(
        primary_key: Self::Id,
        conn: &mut C,
    ) -> impl Future<Output = Result<Option<Self>, diesel::result::Error>>;
}

/// Asynchronous variant of the `BoundedRead` trait.
pub trait AsyncBoundedRead<C: AsyncConnection>: Sized {
    /// Loads the rows in the provided range.
    ///
    /// # Arguments
    ///
    /// * `offeset` - The offset of the first row to be loaded.
    /// * `limit` - The maximum number of rows to be loaded.
    /// * `conn` - A mutable reference to an asynchronous connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn bounded_read_async(
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> impl Future<Output = Result<Vec<Self>, diesel::result::Error>>;

    /// Loads all the rows in the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to an asynchronous connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the rows fails.
    fn read_all_async(
        conn: &mut C,
    ) -> impl Future<Output = Result<Vec<Self>, diesel::result::Error>> {
        Self::bounded_read_async(0u16, u16::MAX, conn)
    }
}
