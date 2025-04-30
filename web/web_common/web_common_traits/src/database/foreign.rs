//! Traits relative to the presence of foreign keys in a struct.

use super::Row;
use crate::crud::{CRUD, Connector};

/// Trait for a struct that has a foreign key.
pub trait Foreign<T> {
    /// The connection type of the table.
    type Conn;

    /// Returns the foreign key.
    fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<Output = Result<T, diesel::result::Error>>;
}

/// Trait for a struct that has one or more foreign keys.
pub trait HasForeignKeys {
    /// The type of foreign keys associated with the struct.
    type ForeignKeys: ForeignKeys;
    /// The type of rows associated with the struct.
    type Row: Row;

    /// Submits the load request for the foreign keys to the provided connector.
    ///
    /// # Arguments
    ///
    /// * `connector` - A reference to the connector to be used for loading
    ///   foreign keys.
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: Connector<Row = Self::Row>;

    /// Returns whether the foreign keys are loaded.
    ///
    /// # Arguments
    ///
    /// * `foreign_keys` - A reference to the foreign keys to check.
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool;

    /// Updates the foreign keys in the struct.
    ///
    /// # Arguments
    ///
    /// * `foreign_keys` - A mutable reference to the foreign keys to be
    ///   updated.
    /// * `row` - The row to be updated.
    /// * `crud` - The CRUD operation to be performed.
    fn update(&self, foreign_keys: &mut Self::ForeignKeys, row: Self::Row, crud: CRUD) -> bool;
}

/// Trait for a struct representing a set of foreign key structs.
pub trait ForeignKeys: Default {}
