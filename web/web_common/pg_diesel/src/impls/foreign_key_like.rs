//! Submodule implementing the
//! [`ForeignKeyLike`](sql_traits::prelude::ForeignKeyLike) trait for the
//! [`KeyColumnUsage`] struct.

use sql_traits::traits::ForeignKeyLike;

use crate::{
    PgDatabase,
    models::{Column, KeyColumnUsage, Table},
};

impl ForeignKeyLike for KeyColumnUsage {
    type Column = Column;
    type Table = Table;
    type Database = PgDatabase;

    fn foreign_key_name(&self) -> Option<&str> {
        Some(&self.constraint_name)
    }

    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        database.foreign_key_metadata(self).referenced_table()
    }

    fn on_delete_cascade(&self, database: &Self::Database) -> bool {
        database.foreign_key_metadata(self).on_delete_cascade()
    }

    fn match_kind(&self, database: &Self::Database) -> sqlparser::ast::MatchKind {
        database.foreign_key_metadata(self).match_kind()
    }

    fn host_columns(
        &self,
        database: &Self::Database,
        _host_table: &Self::Table,
    ) -> impl Iterator<Item = Self::Column> {
        database.foreign_key_metadata(self).host_columns().iter().cloned()
    }

    fn referenced_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        database.foreign_key_metadata(self).referenced_columns().iter().cloned()
    }
}
