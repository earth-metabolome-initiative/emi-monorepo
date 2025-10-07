//! Submodule providing a trait for describing SQL Table-like entities.

use crate::traits::{CheckConstraintLike, ColumnLike, DatabaseLike, UniqueIndexLike};

/// A trait for types that can be treated as SQL tables.
pub trait TableLike {
    /// The database type the table belongs to.
    type Database: DatabaseLike<Table = Self, Column = Self::Column>;
    /// The column type of the table.
    type Column: ColumnLike;
    /// The check constraint type of the table.
    type CheckConstraint: CheckConstraintLike;
    /// The unique index type of the table.
    type UniqueIndex: UniqueIndexLike;

    /// Returns the name of the table.
    fn table_name(&self) -> &str;

    /// Iterates over the columns of the column using the provided schema.
    fn columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column>;

    /// Iterates over the check constraints of the table using the provided
    /// schema.
    fn check_constraints(
        &self,
        database: &Self::Database,
    ) -> impl Iterator<Item = Self::CheckConstraint>;

    /// Iterates over the unique indexes of the table using the provided
    /// schema.
    fn unique_indexes(&self, database: &Self::Database) -> impl Iterator<Item = Self::UniqueIndex>;
}
