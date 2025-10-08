//! Submodule implementing the [`TableLike`](sql_traits::prelude::TableLike)
//! trait for the [`Table`] struct.

use sql_traits::traits::TableLike;

use crate::{
    PgDatabase,
    models::{CheckConstraint, Column, KeyColumnUsage, PgIndex},
};

impl TableLike for crate::models::Table {
    type Column = Column;
    type CheckConstraint = CheckConstraint;
    type ForeignKey = KeyColumnUsage;
    type UniqueIndex = PgIndex;
    type Database = PgDatabase;

    fn table_name(&self) -> &str {
        &self.table_name
    }

    fn table_schema(&self) -> Option<&str> {
        Some(&self.table_schema)
    }

    fn columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        database.table_metadata(self).columns().iter().cloned()
    }

    fn primary_key_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        database.table_metadata(self).primary_key_columns().iter().cloned()
    }

    fn foreign_keys(&self, database: &Self::Database) -> impl Iterator<Item = Self::ForeignKey> {
        database.table_metadata(self).foreign_keys().iter().cloned()
    }

    fn check_constraints(
        &self,
        database: &Self::Database,
    ) -> impl Iterator<Item = Self::CheckConstraint> {
        database.table_metadata(self).check_constraints().iter().cloned()
    }

    fn unique_indices(&self, database: &Self::Database) -> impl Iterator<Item = Self::UniqueIndex> {
        database.table_metadata(self).unique_indices().iter().cloned()
    }
}
