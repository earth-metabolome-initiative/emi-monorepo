//! Submodule defining the `Constrainer` trait, which defines an object that
//! executes registered constraints while visiting a schema.

use crate::{
    error::Error,
    traits::{ColumnConstraint, ConstrainableTable, TableConstraint},
};

mod generic_constrainer;
pub use generic_constrainer::GenericConstrainer;

/// Trait for types that define a constrainer object.
pub trait Constrainer: Default {
    /// Registers a table constraint to be applied to a table.
    fn register_table_constraint(&mut self, constraint: Box<dyn TableConstraint>);

    /// Registers a column constraint to be applied to a column.
    fn register_column_constraint(&mut self, constraint: Box<dyn ColumnConstraint>);

    /// Returns an iterator over all registered table constraints.
    fn table_constraints(&self) -> impl Iterator<Item = &dyn TableConstraint>;

    /// Returns an iterator over all registered column constraints.
    fn column_constraints(&self) -> impl Iterator<Item = &dyn ColumnConstraint>;

    /// Encounters a table and applies all registered table constraints to it.
    fn encounter_table(&self, table: &dyn ConstrainableTable) -> Result<(), Error> {
        self.table_constraints().try_for_each(|constraint| constraint.validate_table(table))
    }

    /// Encounters a column and applies all registered column constraints to it.
    fn encounter_column(
        &self,
        column: &dyn crate::traits::ConstrainableColumn,
    ) -> Result<(), Error> {
        self.column_constraints().try_for_each(|constraint| constraint.validate_column(column))
    }

    /// Validates the provided schema by applying all registered constraints to
    /// its DB entities.
    fn validate_schema<S: crate::traits::Schema>(&self, schema: &S) -> Result<(), Error> {
        for table in schema.tables() {
            self.encounter_table(table)?;
            for column in schema.columns(table) {
                self.encounter_column(&column)?;
            }
        }
        Ok(())
    }
}
