//! Submodule defining a generic constrainer for SQL constraints.

use crate::traits::Constrainer;

#[derive(Default)]
/// A generic constrainer that holds and applies table constraints.
pub struct GenericConstrainer {
    /// The registered table constraints.
    table_constraints: Vec<Box<dyn crate::traits::TableConstraint>>,
    /// The registered column constraints.
    column_constraints: Vec<Box<dyn crate::traits::ColumnConstraint>>,
}

impl Constrainer for GenericConstrainer {
    fn register_table_constraint(&mut self, constraint: Box<dyn crate::traits::TableConstraint>) {
        self.table_constraints.push(constraint);
    }

    fn table_constraints(&self) -> impl Iterator<Item = &dyn crate::traits::TableConstraint> {
        self.table_constraints.iter().map(|c| c.as_ref())
    }

    fn column_constraints(&self) -> impl Iterator<Item = &dyn crate::traits::ColumnConstraint> {
        self.column_constraints.iter().map(|c| c.as_ref())
    }

    fn register_column_constraint(&mut self, constraint: Box<dyn crate::traits::ColumnConstraint>) {
        self.column_constraints.push(constraint);
    }
}
