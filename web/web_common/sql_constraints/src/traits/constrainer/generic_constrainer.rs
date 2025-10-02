//! Submodule defining a generic constrainer for SQL constraints.

use sql_traits::traits::{ColumnLike, TableLike};

use crate::traits::Constrainer;

/// A generic constrainer that holds and applies table constraints.
pub struct GenericConstrainer<T, C> {
    /// The registered table constraints.
    table_constraints: Vec<Box<dyn crate::traits::TableConstraint<Table = T>>>,
    /// The registered column constraints.
    column_constraints: Vec<Box<dyn crate::traits::ColumnConstraint<Column = C>>>,
}

impl<T, C> Default for GenericConstrainer<T, C> {
    fn default() -> Self {
        Self { table_constraints: Vec::new(), column_constraints: Vec::new() }
    }
}

impl<T: TableLike, C: ColumnLike> Constrainer for GenericConstrainer<T, C> {
    type Table = T;
    type Column = C;

    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::TableConstraint<Table = Self::Table>>,
    ) {
        self.table_constraints.push(constraint);
    }

    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::TableConstraint<Table = Self::Table>> {
        self.table_constraints.iter().map(|c| c.as_ref())
    }

    fn column_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::ColumnConstraint<Column = Self::Column>> {
        self.column_constraints.iter().map(|c| c.as_ref())
    }

    fn register_column_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::ColumnConstraint<Column = Self::Column>>,
    ) {
        self.column_constraints.push(constraint);
    }
}
