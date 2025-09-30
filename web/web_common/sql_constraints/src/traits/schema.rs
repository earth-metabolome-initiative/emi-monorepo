//! Submodule providing the `Schema` trait, which defines a struct
//! characterized by being able to represent DB objects. It
//! also is built with the assumption that possibly some of the SQL objects
//! encountered during the visit (e.g. foreign tables in `FOREIGN KEY`
//! constraints) may not be defined via the provided SQL statements but some
//! other means (e.g. pre-existing tables in the DB), and thus will refrain from
//! enforcing strict checks on the existence of such objects.

use crate::traits::{ConstrainableColumn, ConstrainableTable};

/// Trait for types that define a dynamic SQL schema.
pub trait Schema {
    /// Type of the tables in the schema.
    type TableType: ConstrainableTable;
    /// Type of the columns in the schema.
    type ColumnType: ConstrainableColumn;

    /// Iterates over the tables defined in the schema.
    fn tables(&self) -> impl Iterator<Item = &Self::TableType>;

    /// Iterates over the columns of the provided table.
    fn columns(&self, table: &Self::TableType) -> impl Iterator<Item = Self::ColumnType>;
}
