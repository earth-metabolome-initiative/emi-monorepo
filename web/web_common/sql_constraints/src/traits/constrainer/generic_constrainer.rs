//! Submodule defining a generic constrainer for SQL constraints.

use sql_traits::traits::DatabaseLike;

use crate::traits::Constrainer;

/// A generic constrainer that holds and applies table constraints.
pub struct GenericConstrainer<DB: DatabaseLike> {
    /// The registered table constraints.
    table_constraints:
        Vec<Box<dyn crate::traits::TableConstraint<Table = DB::Table, Database = DB>>>,
    /// The registered column constraints.
    column_constraints: Vec<Box<dyn crate::traits::ColumnConstraint<Column = DB::Column>>>,
}

impl<DB: DatabaseLike> Default for GenericConstrainer<DB> {
    fn default() -> Self {
        Self { table_constraints: Vec::new(), column_constraints: Vec::new() }
    }
}

impl<DB: DatabaseLike> Constrainer for GenericConstrainer<DB> {
    type Table = DB::Table;
    type Column = DB::Column;
    type Database = DB;

    fn register_table_constraint(
        &mut self,
        constraint: Box<
            dyn crate::traits::TableConstraint<Table = Self::Table, Database = Self::Database>,
        >,
    ) {
        self.table_constraints.push(constraint);
    }

    fn table_constraints(
        &self,
    ) -> impl Iterator<
        Item = &dyn crate::traits::TableConstraint<Table = Self::Table, Database = Self::Database>,
    > {
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
