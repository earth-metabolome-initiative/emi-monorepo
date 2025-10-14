//! Submodule defining the `ForeignKeyConstraint` trait, which defines a
//! constraint which applies to an object that implements the `ForeignKeyLike`
//! trait.

use sql_traits::traits::{DatabaseLike, ForeignKeyLike, TableLike};

use crate::error::Error;

/// Trait for types that define a foreign key constraint object.
pub trait ForeignKeyConstraint {
    /// The associated type for the foreign-key-like object that this constraint
    /// applies to.
    type ForeignKey: ForeignKeyLike<Table = Self::Table, Database = Self::Database>;
    /// The database type that this constraint applies to.
    type Database: DatabaseLike<ForeignKey = Self::ForeignKey>;
    /// The table type that this constraint applies to.
    type Table: TableLike<Database = Self::Database, ForeignKey = Self::ForeignKey>;

    /// Validates that the given foreign key satisfies the constraint.
    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &Self::ForeignKey,
    ) -> Result<(), Error>;
}
