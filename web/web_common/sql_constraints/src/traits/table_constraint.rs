//! Submodule defining the `TableConstraint` trait, which defines a constraint
//! which applies to an object that implements the `ConstrainableTable` trait.

use sql_traits::traits::TableLike;

use crate::{error::Error, traits::ConstraintFailureInformation};

/// Trait for types that define a table constraint object.
pub trait TableConstraint {
    /// The associated type for the table-like object that this constraint
    /// applies to.
    type Table: TableLike;

    /// Returns information about the failure of this constraint.
    fn table_error_information(
        &self,
        context: &Self::Table,
    ) -> Box<dyn ConstraintFailureInformation>;

    /// Validates that the given table satisfies the constraint.
    fn validate_table(&self, table: &Self::Table) -> Result<(), Error>;
}
