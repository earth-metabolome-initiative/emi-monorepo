//! Submodule defining the `TableConstraint` trait, which defines a constraint
//! which applies to an object that implements the `ConstrainableTable` trait.

use sql_traits::traits::DatabaseLike;

use crate::{error::Error, traits::ConstraintFailureInformation};

/// Trait for types that define a table constraint object.
pub trait TableConstraint {
    /// The database type that this constraint applies to.
    type Database: DatabaseLike;

    /// Returns information about the failure of this constraint.
    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn ConstraintFailureInformation>;

    /// Validates that the given table satisfies the constraint.
    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error>;
}
