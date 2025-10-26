//! Submodule defining a generic constrainer for SQL constraints.

use sql_traits::traits::DatabaseLike;

use crate::traits::Constrainer;

/// A generic constrainer that holds and applies table constraints.
pub struct GenericConstrainer<DB: DatabaseLike> {
    /// The registered table constraints.
    table_constraints: Vec<Box<dyn crate::traits::TableConstraint<Database = DB>>>,
    /// The registered column constraints.
    column_constraints: Vec<Box<dyn crate::traits::ColumnConstraint<Column = DB::Column>>>,
    /// The registered foreign key constraints.
    foreign_key_constraints: Vec<Box<dyn crate::traits::ForeignKeyConstraint<Database = DB>>>,
}

impl<DB: DatabaseLike> Default for GenericConstrainer<DB> {
    fn default() -> Self {
        Self {
            table_constraints: Vec::new(),
            column_constraints: Vec::new(),
            foreign_key_constraints: Vec::new(),
        }
    }
}

impl<DB: DatabaseLike> Constrainer for GenericConstrainer<DB> {
    type Database = DB;

    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::TableConstraint<Database = Self::Database>> {
        self.table_constraints.iter().map(|c| c.as_ref())
    }

    fn column_constraints(
        &self,
    ) -> impl Iterator<
        Item = &dyn crate::traits::ColumnConstraint<
            Column = <Self::Database as DatabaseLike>::Column,
        >,
    > {
        self.column_constraints.iter().map(|c| c.as_ref())
    }

    fn foreign_key_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::prelude::ForeignKeyConstraint<Database = Self::Database>>
    {
        self.foreign_key_constraints.iter().map(|c| c.as_ref())
    }

    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::TableConstraint<Database = Self::Database>>,
    ) {
        self.table_constraints.push(constraint);
    }

    fn register_column_constraint(
        &mut self,
        constraint: Box<
            dyn crate::traits::ColumnConstraint<Column = <Self::Database as DatabaseLike>::Column>,
        >,
    ) {
        self.column_constraints.push(constraint);
    }

    fn register_foreign_key_constraint(
        &mut self,
        constraint: Box<dyn crate::prelude::ForeignKeyConstraint<Database = Self::Database>>,
    ) {
        self.foreign_key_constraints.push(constraint);
    }
}
