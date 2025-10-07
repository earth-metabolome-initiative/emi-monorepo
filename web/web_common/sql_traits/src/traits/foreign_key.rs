//! Submodule definining the `ForeignKeyLike` trait for SQL foreign keys.

use crate::traits::{ColumnLike, DatabaseLike, TableLike};

/// A foreign key constraint is a rule that specifies a relationship between
/// two tables. This trait represents such a foreign key constraint in a
/// database-agnostic way.
pub trait ForeignKeyLike {
    /// The column type associated with the foreign key.
    type Column: ColumnLike;
    /// The table type associated with the foreign key.
    type Table: TableLike<Column = Self::Column, Database = Self::Database>;
    /// The database type associated with the foreign key.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column>;

    /// Returns the host table that contains the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn host_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table;

    /// Returns the referenced table that the foreign key points to.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table;

    /// Returns an iterator over the columns in the host table that are part of
    /// the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn host_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;

    /// Returns an iterator over the columns in the referenced table that are
    /// part of the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn referenced_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;
}
