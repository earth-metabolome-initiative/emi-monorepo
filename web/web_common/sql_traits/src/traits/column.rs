//! Submodule providing a trait for describing SQL Column-like entities.

use std::hash::Hash;

/// A trait for types that can be treated as SQL columns.
pub trait ColumnLike: Hash + Eq {
    /// Returns the name of the column.
    fn column_name(&self) -> &str;

    /// Returns the data type of the column as a string.
    fn data_type(&self) -> String;

    /// Returns whether the column is nullable.
    fn is_nullable(&self) -> bool;
}
