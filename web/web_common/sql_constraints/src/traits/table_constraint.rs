//! Submodule defining the `TableConstraint` trait, which defines a constraint which applies to an object
//! that implements the `ConstrainableTable` trait.

use crate::{
    error::Error,
    traits::{ConstrainableTable, Constraint},
};

/// Trait for types that define a table constraint object.
pub trait TableConstraint: Constraint {
    /// Validates that the given table satisfies the constraint.
    fn validate_table(&self, table: &dyn ConstrainableTable) -> Result<(), Error>;
}
