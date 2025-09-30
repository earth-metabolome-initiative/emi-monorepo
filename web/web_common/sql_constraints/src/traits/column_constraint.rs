//! Submodule defining the `ColumnConstraint` trait, which defines a constraint
//! which applies to an object that implements the `ConstrainableColumn` trait.

use crate::{
    error::Error,
    traits::{ConstrainableColumn, Constraint},
};

/// Trait for types that define a column constraint object.
pub trait ColumnConstraint: Constraint {
    /// Validates that the given column satisfies the constraint.
    fn validate_column(&self, column: &dyn ConstrainableColumn) -> Result<(), Error>;
}
