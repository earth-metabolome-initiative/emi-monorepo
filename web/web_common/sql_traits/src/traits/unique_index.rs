//! Submodule definining the `UniqueIndexLike` trait for SQL unique
//! indexes.

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, TableLike};

/// A unique index is a rule that specifies that the values in a column
/// (or a group of columns) must be unique across all rows in a table.
/// This trait represents such a unique index in a database-agnostic way.
pub trait UniqueIndexLike {
    /// The table type the unique index belongs to.
    type Table: TableLike<Database = Self::Database, UniqueIndex = Self>;
    /// The database type the unique index belongs to.
    type Database: DatabaseLike<Table = Self::Table>;

    /// Returns the expression of the unique index as an SQL AST node.
    fn expression(&self, database: &Self::Database) -> Expr;
}
