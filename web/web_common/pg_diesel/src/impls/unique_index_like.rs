//! Implementation of [`UniqueIndexLike`] for [`PgIndex`].
//!
//! This module implements the
//! [`UniqueIndexLike`](sql_traits::prelude::UniqueIndexLike) trait for the
//! [`PgIndex`] model from `pg_catalog.pg_index`, enabling generic introspection
//! of unique indexes and primary keys.
//!
//! The implementation provides access to:
//! - The table that the index is defined on
//! - The parsed index expression (from `pg_get_indexdef`)
//!
//! Metadata is provided by [`UniqueIndexMetadata`] from the `sql_traits` crate.

use sql_traits::{
    structs::metadata::UniqueIndexMetadata,
    traits::{Metadata, UniqueIndexLike},
};
use sqlparser::ast::Expr;

use crate::{PgDatabase, models::PgIndex};

impl Metadata for PgIndex {
    type Meta = UniqueIndexMetadata<Self>;
}

impl UniqueIndexLike for PgIndex {
    type DB = PgDatabase;

    fn table<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as sql_traits::traits::DatabaseLike>::Table
    where
        Self: 'db,
    {
        database.index_metadata(self).table()
    }

    fn expression<'db>(&'db self, database: &'db Self::DB) -> &'db Expr
    where
        Self: 'db,
    {
        database.index_metadata(self).expression()
    }
}
