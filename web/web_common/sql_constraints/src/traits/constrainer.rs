//! Submodule defining the `Constrainer` trait, which defines an object that
//! executes registered constraints while visiting a schema.

use crate::{
    error::Error,
    traits::{ColumnConstraint, ForeignKeyConstraint, TableConstraint},
};

pub mod generic_constrainer;
pub use generic_constrainer::GenericConstrainer;
pub mod default_constrainer;
pub use default_constrainer::DefaultConstrainer;
use sql_traits::traits::{DatabaseLike, TableLike};

/// Trait for types that define a constrainer object.
pub trait Constrainer: Default {
    /// Associated database type for the constrainer.
    type Database: DatabaseLike;

    /// Registers a table constraint to be applied to a table.
    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn TableConstraint<Database = Self::Database>>,
    );

    /// Registers a column constraint to be applied to a column.
    fn register_column_constraint(
        &mut self,
        constraint: Box<dyn ColumnConstraint<Column = <Self::Database as DatabaseLike>::Column>>,
    );

    /// Registers a foreign key constraint to be applied to a foreign key.
    fn register_foreign_key_constraint(
        &mut self,
        constraint: Box<dyn ForeignKeyConstraint<Database = Self::Database>>,
    );

    /// Returns an iterator over all registered table constraints.
    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn TableConstraint<Database = Self::Database>>;

    /// Returns an iterator over all registered column constraints.
    fn column_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn ColumnConstraint<Column = <Self::Database as DatabaseLike>::Column>>;

    /// Returns an iterator over all registered foreign key constraints.
    fn foreign_key_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn ForeignKeyConstraint<Database = Self::Database>>;

    /// Encounters a table and applies all registered table constraints to it.
    ///
    /// # Errors
    ///
    /// Returns an error if any table constraint is violated.
    fn encounter_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error> {
        self.table_constraints()
            .try_for_each(|constraint| constraint.validate_table(database, table))
    }

    /// Encounters a column and applies all registered column constraints to it.
    ///
    /// # Errors
    ///
    /// Returns an error if any column constraint is violated.
    fn encounter_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), Error> {
        self.column_constraints()
            .try_for_each(|constraint| constraint.validate_column(database, column))
    }

    /// Encounters a foreign key and applies all registered foreign key
    /// constraints to it.
    ///
    /// # Errors
    ///
    /// Returns an error if any foreign key constraint is violated.
    fn encounter_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), Error> {
        self.foreign_key_constraints()
            .try_for_each(|constraint| constraint.validate_foreign_key(database, foreign_key))
    }

    /// Validates the provided schema by applying all registered constraints to
    /// its DB entities.
    ///
    /// # Errors
    ///
    /// Returns an error if any constraint is violated.
    fn validate_schema(&self, database: &Self::Database) -> Result<(), Error> {
        for table in database.tables() {
            self.encounter_table(database, table)?;
            for column in table.columns(database) {
                self.encounter_column(database, column)?;
            }
            for foreign_key in table.foreign_keys(database) {
                self.encounter_foreign_key(database, foreign_key)?;
            }
        }
        Ok(())
    }
}
