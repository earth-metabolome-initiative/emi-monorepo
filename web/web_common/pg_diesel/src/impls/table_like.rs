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

    fn columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).columns().iter()
    }

    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).primary_key_columns().iter()
    }

    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        database.table_metadata(self).foreign_keys().iter()
    }

    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::CheckConstraint>
    where
        Self: 'db,
    {
        database.table_metadata(self).check_constraints().iter()
    }

    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::UniqueIndex>
    where
        Self: 'db,
    {
        database.table_metadata(self).unique_indices().iter()
    }
}
