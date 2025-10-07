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

    /// Returns the name of the foreign key, if it has one.
    fn foreign_key_name(&self) -> Option<&str>;

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
    /// * `host_table` - A reference to the host table that contains the foreign
    ///   key.
    fn host_columns(
        &self,
        database: &Self::Database,
        host_table: &Self::Table,
    ) -> impl Iterator<Item = Self::Column>;

    /// Returns an iterator over the columns in the referenced table that are
    /// part of the foreign key.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn referenced_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;

    /// Returns whether the foreign key references the primary key of the
    /// referenced table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the foreign
    ///   key belongs.
    fn is_referenced_primary_key(&self, database: &Self::Database) -> bool {
        let referenced_table = self.referenced_table(database);
        let mut pk_columns = referenced_table.primary_key_columns(database);
        let mut fk_columns = self.referenced_columns(database);

        while let (Some(fk_col), Some(pk_col)) = (fk_columns.next(), pk_columns.next()) {
            if fk_col != pk_col {
                return false;
            }
        }

        // We check that there are no remaining columns in either iterator.
        fk_columns.next().is_none() && pk_columns.next().is_none()
    }
}
