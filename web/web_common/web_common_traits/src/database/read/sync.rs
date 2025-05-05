//! Submodule providing traits and blanked implementations for the `Read`
//! operation.

mod postgres;
mod sqlite;
use diesel::Identifiable;

/// The `Read` trait
pub trait Read<C>: Sized + Identifiable {
    /// Loads the row in a table.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to a load connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn read(primary_key: Self::Id, conn: &mut C) -> Result<Option<Self>, diesel::result::Error>;
}

/// The `BoundedRead` trait
pub trait BoundedRead<C>: Sized {
    /// Loads the rows in the provided range.
    ///
    /// # Arguments
    ///
    /// * `offeset` - The offset of the first row to be loaded.
    /// * `limit` - The maximum number of rows to be loaded.
    /// * `conn` - A mutable reference to a load connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn bounded_read(
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>;
}
