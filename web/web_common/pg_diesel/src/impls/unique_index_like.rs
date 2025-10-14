//! Submodule implementing the
//! [`UniqueIndexLike`](sql_traits::prelude::UniqueIndexLike) trait for the
//! [`PgIndex`] struct.

use sql_traits::{
    structs::metadata::UniqueIndexMetadata,
    traits::{Metadata, UniqueIndexLike},
};
use sqlparser::ast::Expr;

use crate::{
    PgDatabase,
    models::{Column, PgIndex, Table},
};

impl Metadata for PgIndex {
    type Meta = UniqueIndexMetadata<Self>;
}

impl UniqueIndexLike for PgIndex {
    type Table = Table;
    type Column = Column;
    type Database = PgDatabase;

    fn table<'db>(&'db self, database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db,
    {
        database.index_metadata(self).table()
    }

    fn expression<'db>(&'db self, database: &'db Self::Database) -> &'db Expr
    where
        Self: 'db,
    {
        database.index_metadata(self).expression()
    }
}
