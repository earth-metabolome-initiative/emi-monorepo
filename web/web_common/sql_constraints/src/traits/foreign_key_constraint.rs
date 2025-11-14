//! Submodule defining the `ForeignKeyConstraint` trait, which defines a
//! constraint which applies to an object that implements the `ForeignKeyLike`
//! trait.

use sql_traits::traits::DatabaseLike;

use crate::error::Error;

/// Trait for types that define a foreign key constraint object.
pub trait ForeignKeyConstraint {
    /// The database type that this constraint applies to.
    type Database: DatabaseLike;

    /// Validates that the given foreign key satisfies the constraint.
    ///
    /// # Errors
    ///
    /// Returns an error if the foreign key violates this constraint.
    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), Error>;
}
