//! Submodule defining the `Constrainer` trait, which defines an object that
//! executes registered constraints while visiting a schema.

use crate::{
    error::Error,
    traits::{ColumnConstraint, ForeignKeyConstraint, TableConstraint},
};

mod generic_constrainer;
pub use generic_constrainer::GenericConstrainer;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

/// Trait for types that define a constrainer object.
pub trait Constrainer: Default {
    /// Associated table type for the constrainer.
    type Table: TableLike<Database = Self::Database, Column = Self::Column, ForeignKey = Self::ForeignKey>;
    /// Associated column type for the constrainer.
    type Column: ColumnLike;
    /// Associated foreign key type for the constrainer.
    type ForeignKey: ForeignKeyLike<Table = Self::Table, Column = Self::Column, Database = Self::Database>;
    /// Associated database type for the constrainer.
    type Database: DatabaseLike<Table = Self::Table, Column = Self::Column, ForeignKey = Self::ForeignKey>;

    /// Registers a table constraint to be applied to a table.
    fn register_table_constraint(
        &mut self,
        constraint: Box<dyn TableConstraint<Table = Self::Table, Database = Self::Database>>,
    );

    /// Registers a column constraint to be applied to a column.
    fn register_column_constraint(
        &mut self,
        constraint: Box<dyn ColumnConstraint<Column = Self::Column>>,
    );

    /// Registers a foreign key constraint to be applied to a foreign key.
    fn register_foreign_key_constraint(
        &mut self,
        constraint: Box<
            dyn ForeignKeyConstraint<
                    ForeignKey = Self::ForeignKey,
                    Database = Self::Database,
                    Table = Self::Table,
                >,
        >,
    );

    /// Returns an iterator over all registered table constraints.
    fn table_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn TableConstraint<Table = Self::Table, Database = Self::Database>>;

    /// Returns an iterator over all registered column constraints.
    fn column_constraints(
        &self,
    ) -> impl Iterator<Item = &dyn ColumnConstraint<Column = Self::Column>>;

    /// Returns an iterator over all registered foreign key constraints.
    fn foreign_key_constraints(
        &self,
    ) -> impl Iterator<
        Item = &dyn ForeignKeyConstraint<
            ForeignKey = Self::ForeignKey,
            Database = Self::Database,
            Table = Self::Table,
        >,
    >;

    /// Encounters a table and applies all registered table constraints to it.
    fn encounter_table(&self, database: &Self::Database, table: &Self::Table) -> Result<(), Error> {
        self.table_constraints()
            .try_for_each(|constraint| constraint.validate_table(database, table))
    }

    /// Encounters a column and applies all registered column constraints to it.
    fn encounter_column(&self, column: &Self::Column) -> Result<(), Error> {
        self.column_constraints().try_for_each(|constraint| constraint.validate_column(column))
    }

    /// Encounters a foreign key and applies all registered foreign key
    /// constraints to it.
    fn encounter_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &Self::ForeignKey,
    ) -> Result<(), Error> {
        self.foreign_key_constraints()
            .try_for_each(|constraint| constraint.validate_foreign_key(database, foreign_key))
    }

    /// Validates the provided schema by applying all registered constraints to
    /// its DB entities.
    fn validate_schema(&self, database: &Self::Database) -> Result<(), Error> {
        for table in database.tables() {
            self.encounter_table(database, table)?;
            for column in table.columns(database) {
                self.encounter_column(&column)?;
            }
            for foreign_key in table.foreign_keys(database) {
                self.encounter_foreign_key(database, &foreign_key)?;
            }
        }
        Ok(())
    }
}
