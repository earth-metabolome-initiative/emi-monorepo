//! Submodule defining a generic constrainer for SQL constraints.

use sql_traits::traits::DatabaseLike;

use crate::traits::Constrainer;

/// A generic constrainer that holds and applies table constraints.
pub struct GenericConstrainer<DB: DatabaseLike> {
    /// The registered table constraints.
    tables: Vec<Box<dyn crate::traits::TableConstraint<Database = DB>>>,
    /// The registered column constraints.
    columns: Vec<Box<dyn crate::traits::ColumnConstraint<Column = DB::Column>>>,
    /// The registered foreign key constraints.
    foreign_keys: Vec<Box<dyn crate::traits::ForeignKeyConstraint<Database = DB>>>,
}

impl<DB: DatabaseLike> Default for GenericConstrainer<DB> {
    fn default() -> Self {
        Self { tables: Vec::new(), columns: Vec::new(), foreign_keys: Vec::new() }
    }
}

impl<DB: DatabaseLike> Constrainer for GenericConstrainer<DB> {
    type Database = DB;

    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::TableConstraint<Database = Self::Database>> {
        self.tables.iter().map(AsRef::as_ref)
    }

    fn column_constraints(
        &self,
    ) -> impl Iterator<
        Item = &dyn crate::traits::ColumnConstraint<
            Column = <Self::Database as DatabaseLike>::Column,
        >,
    > {
        self.columns.iter().map(AsRef::as_ref)
    }

    fn foreign_key_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::ForeignKeyConstraint<Database = Self::Database>>
    {
        self.foreign_keys.iter().map(AsRef::as_ref)
    }

    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::TableConstraint<Database = Self::Database>>,
    ) {
        self.tables.push(constraint);
    }

    fn register_column_constraint(
        &mut self,
        constraint: Box<
            dyn crate::traits::ColumnConstraint<Column = <Self::Database as DatabaseLike>::Column>,
        >,
    ) {
        self.columns.push(constraint);
    }

    fn register_foreign_key_constraint(
        &mut self,
        constraint: Box<dyn crate::traits::ForeignKeyConstraint<Database = Self::Database>>,
    ) {
        self.foreign_keys.push(constraint);
    }
}
