//! Submodule providing a trait for describing SQL Database-like entities.

use crate::traits::{ColumnLike, TableLike};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike {
    /// Type of the tables in the schema.
    type Table: TableLike<Column = Self::Column, Database = Self>;
    /// Type of the columns in the schema.
    type Column: ColumnLike;

    /// Iterates over the tables defined in the schema.
    fn tables(&self) -> impl Iterator<Item = &Self::Table>;
}
