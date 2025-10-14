//! Submodule implementing the
//! [`ForeignKeyLike`](sql_traits::prelude::ForeignKeyLike) trait for the
//! [`KeyColumnUsage`] struct.

use sql_traits::traits::{ForeignKeyLike, Metadata};

use crate::{
    PgDatabase,
    database::KeyColumnUsageMetadata,
    models::{Column, KeyColumnUsage, PgIndex, Table},
};

impl Metadata for KeyColumnUsage {
    type Meta = KeyColumnUsageMetadata;
}

impl ForeignKeyLike for KeyColumnUsage {
    type Column = Column;
    type Table = Table;
    type Database = PgDatabase;
    type UniqueIndex = PgIndex;

    fn foreign_key_name(&self) -> Option<&str> {
        Some(&self.constraint_name)
    }

    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        database.foreign_key_metadata(self).referenced_table()
    }

    fn host_table<'db>(&'db self, database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db,
    {
        database.foreign_key_metadata(self).host_table()
    }

    fn on_delete_cascade(&self, database: &Self::Database) -> bool {
        database.foreign_key_metadata(self).on_delete_cascade()
    }

    fn match_kind(&self, database: &Self::Database) -> sqlparser::ast::MatchKind {
        database.foreign_key_metadata(self).match_kind()
    }

    fn host_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.foreign_key_metadata(self).host_columns().iter()
    }

    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.foreign_key_metadata(self).referenced_columns().iter()
    }
}
