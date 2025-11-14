//! Submodule defining the `ColumnConstraint` trait, which defines a constraint
//! which applies to an object that implements the `ConstrainableColumn` trait.

use sql_traits::traits::ColumnLike;

use crate::error::Error;

/// Trait for types that define a column constraint object.
pub trait ColumnConstraint {
    /// The column type that this constraint applies to.
    type Column: ColumnLike;

    /// Returns information about the failure of this constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query additional
    ///   information needed for the error message.
    /// * `context` - The column that failed the constraint.
    fn column_error_information(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        context: &Self::Column,
    ) -> Box<dyn crate::traits::ConstraintFailureInformation>;

    /// Validates that the given column satisfies the constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query additional
    ///   column information from.
    /// * `column` - The column to validate.
    ///
    /// # Errors
    ///
    /// Returns an error if the column violates this constraint.
    fn validate_column(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Result<(), Error>;
}
