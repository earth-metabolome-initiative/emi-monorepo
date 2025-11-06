//! Submodule providing the `TableMetadata` struct for a [`Table`] model.

use std::rc::Rc;

use crate::models::{CheckConstraint, Column, KeyColumnUsage, PgDescription, PgIndex};

#[derive(Clone, Debug)]
/// Rich metadata about a PostgreSQL table.
///
/// This struct wraps a table model with additional metadata loaded from related
/// system catalog tables, including:
/// - Columns belonging to the table
/// - Check constraints defined on the table
/// - Unique indexes (including primary key)
/// - Foreign keys referencing other tables
/// - Table description from `pg_catalog.pg_description`
///
/// This metadata is constructed during
/// [`PgDatabase`](crate::database::PgDatabase) building and cached for
/// efficient access via the [`TableLike`](sql_traits::traits::TableLike) trait.
pub struct TableMetadata {
    /// The underlying table metadata.
    metadata: sql_traits::structs::TableMetadata<crate::models::Table>,
    /// The description of the table, if any.
    description: Option<PgDescription>,
}

impl TableMetadata {
    /// Creates a new `TableMetadata` instance.
    pub fn new(
        metadata: sql_traits::structs::TableMetadata<crate::models::Table>,
        description: Option<PgDescription>,
    ) -> Self {
        Self { metadata, description }
    }

    /// Returns an iterator over the references of columns of the table.
    pub fn columns(&self) -> impl Iterator<Item = &Column> {
        self.metadata.columns()
    }

    /// Returns an iterator over the Rc of columns of the table.
    pub fn column_rcs(&self) -> impl Iterator<Item = &Rc<Column>> {
        self.metadata.column_rcs()
    }

    /// Returns an iterator over the check constraints of the table.
    pub fn check_constraints(&self) -> impl Iterator<Item = &CheckConstraint> {
        self.metadata.check_constraints()
    }

    /// Returns an iterator over the Rc of check constraints of the table.
    pub fn check_constraint_rcs(&self) -> impl Iterator<Item = &Rc<CheckConstraint>> {
        self.metadata.check_constraint_rcs()
    }

    /// Returns an iterator over the unique indices of the table.
    pub fn unique_indices(&self) -> impl Iterator<Item = &PgIndex> {
        self.metadata.unique_indices()
    }

    /// Returns an iterator over the Rc of unique indices of the table.
    pub fn unique_index_rcs(&self) -> impl Iterator<Item = &Rc<PgIndex>> {
        self.metadata.unique_index_rcs()
    }

    /// Returns an iterator over the foreign keys of the table.
    pub fn foreign_keys(&self) -> impl Iterator<Item = &KeyColumnUsage> {
        self.metadata.foreign_keys()
    }

    /// Returns an iterator over the Rc of foreign keys of the table.
    pub fn foreign_key_rcs(&self) -> impl Iterator<Item = &Rc<KeyColumnUsage>> {
        self.metadata.foreign_key_rcs()
    }

    /// Returns an iterator over the columns composing the primary key of the
    /// table.
    pub fn primary_key_columns(&self) -> impl Iterator<Item = &Column> {
        self.metadata.primary_key_columns()
    }

    /// Returns the description of the table, if any.
    pub fn description(&self) -> Option<&PgDescription> {
        self.description.as_ref()
    }
}
