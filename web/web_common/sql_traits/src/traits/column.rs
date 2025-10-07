//! Submodule providing a trait for describing SQL Column-like entities.

use std::hash::Hash;

/// A trait for types that can be treated as SQL columns.
pub trait ColumnLike: Hash {
    /// Returns the name of the column.
    fn column_name(&self) -> &str;
}
