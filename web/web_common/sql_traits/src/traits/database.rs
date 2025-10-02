//! Submodule providing a trait for describing SQL Database-like entities.

use crate::traits::{ColumnLike, TableLike};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike {
    /// Type of the tables in the schema.
    type Table: TableLike;
    /// Type of the columns in the schema.
    type Column: ColumnLike;

    /// Iterates over the tables defined in the schema.
    fn tables(&self) -> impl Iterator<Item = &Self::Table>;

    /// Iterates over the columns of the provided table.
    fn columns(&self, table: &Self::Table) -> impl Iterator<Item = Self::Column>;
}
