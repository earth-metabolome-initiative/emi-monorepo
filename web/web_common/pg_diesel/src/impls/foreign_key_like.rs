//! Implementation of [`ForeignKeyLike`] for [`KeyColumnUsage`].
//!
//! This module implements the
//! [`ForeignKeyLike`](sql_traits::prelude::ForeignKeyLike) trait
//! for the [`KeyColumnUsage`] model from `information_schema.key_column_usage`,
//! enabling generic introspection of foreign key relationships.
//!
//! The implementation provides access to:
//! - Foreign key constraint name
//! - Host (referencing) table and columns
//! - Referenced (target) table and columns
//! - Referential action rules (ON DELETE CASCADE, MATCH kind)
//!
//! Metadata is loaded from [`KeyColumnUsageMetadata`] which is constructed
//! during database building.

use sql_traits::traits::{ForeignKeyLike, Metadata};

use crate::{PgDatabase, database::KeyColumnUsageMetadata, models::KeyColumnUsage};

impl Metadata for KeyColumnUsage {
    type Meta = KeyColumnUsageMetadata;
}

impl ForeignKeyLike for KeyColumnUsage {
    type DB = PgDatabase;

    fn foreign_key_name(&self) -> Option<&str> {
        Some(&self.constraint_name)
    }

    fn referenced_table<'db>(
        &self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as sql_traits::traits::DatabaseLike>::Table {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .referenced_table()
    }

    fn host_table<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as sql_traits::traits::DatabaseLike>::Table
    where
        Self: 'db,
    {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .host_table()
    }

    fn on_delete_cascade(&self, database: &Self::DB) -> bool {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .on_delete_cascade()
    }

    fn match_kind(&self, database: &Self::DB) -> sqlparser::ast::ConstraintReferenceMatchKind {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .match_kind()
    }

    fn host_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as sql_traits::traits::DatabaseLike>::Column>
    where
        Self: 'db,
    {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .host_columns()
            .iter()
    }

    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as sql_traits::traits::DatabaseLike>::Column>
    where
        Self: 'db,
    {
        database
            .foreign_key_metadata(self)
            .expect("Foreign key must exist in database")
            .referenced_columns()
            .iter()
    }
}
