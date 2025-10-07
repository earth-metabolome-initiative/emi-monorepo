//! Submodule definining the `UniqueIndexLike` trait for SQL unique
//! indexes.

/// A unique index is a rule that specifies that the values in a column
/// (or a group of columns) must be unique across all rows in a table.
/// This trait represents such a unique index in a database-agnostic way.
pub trait UniqueIndexLike {
    /// Returns the clause of the unique index as a string.
    fn clause(&self) -> String;
}
