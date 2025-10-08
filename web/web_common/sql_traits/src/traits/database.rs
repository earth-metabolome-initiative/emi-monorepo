//! Submodule providing a trait for describing SQL Database-like entities.

use crate::traits::{ColumnLike, ForeignKeyLike, TableLike};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike {
    /// Type of the tables in the schema.
    type Table: TableLike<Column = Self::Column, Database = Self, ForeignKey = Self::ForeignKey>;
    /// Type of the columns in the schema.
    type Column: ColumnLike;
    /// Type of the foreign keys in the schema.
    type ForeignKey: ForeignKeyLike<Table = Self::Table, Column = Self::Column, Database = Self>;

    /// Iterates over the tables defined in the schema.
    fn tables(&self) -> impl Iterator<Item = &Self::Table>;

    /// Returns the table with the given (optional) schema and name.
    ///
    /// # Arguments
    ///
    /// * `schema` - Optional schema name of the table.
    /// * `table_name` - Name of the table.
    ///
    /// # Panics
    ///
    /// Panics if the table is not found in the database.
    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table;
}
