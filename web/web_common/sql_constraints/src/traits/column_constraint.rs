//! Submodule defining the `ColumnConstraint` trait, which defines a constraint
//! which applies to an object that implements the `ConstrainableColumn` trait.

use sql_traits::traits::ColumnLike;

use crate::error::Error;

/// Trait for types that define a column constraint object.
pub trait ColumnConstraint {
    /// The associated type for the column-like object that this constraint
    /// applies to.
    type Column: ColumnLike;

    /// Returns information about the failure of this constraint.
    fn column_error_information(
        &self,
        context: &Self::Column,
    ) -> Box<dyn crate::traits::ConstraintFailureInformation>;

    /// Validates that the given column satisfies the constraint.
    fn validate_column(&self, column: &Self::Column) -> Result<(), Error>;
}
