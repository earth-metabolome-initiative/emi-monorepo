//! Submodule defining the `Constrainer` trait, which defines an object that executes registered constraints
//! while visiting a schema.

use crate::{
    error::Error,
    traits::{ConstrainableTable, TableConstraint},
};

/// Trait for types that define a constrainer object.
pub trait Constrainer {
    /// Registers a table constraint to be applied to a table.
    fn register_table_constraint(&mut self, constraint: &dyn TableConstraint);

    /// Returns an iterator over all registered table constraints.
    fn table_constraints(&self) -> impl Iterator<Item = &dyn TableConstraint>;

    /// Encounters a table and applies all registered table constraints to it.
    fn encounter_table<T: ConstrainableTable>(&self, table: &T) -> Result<(), Error> {
        self.table_constraints().try_for_each(|constraint| constraint.validate_table(table))
    }
}
