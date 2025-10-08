//! Submodule implementing the
//! [`UniqueIndexLike`](sql_traits::prelude::UniqueIndexLike) trait for the
//! [`PgIndex`] struct.

use sql_traits::traits::UniqueIndexLike;
use sqlparser::ast::Expr;

use crate::{
    PgDatabase,
    models::{PgIndex, Table},
};

impl UniqueIndexLike for PgIndex {
    type Table = Table;
    type Database = PgDatabase;

    fn expression(&self, database: &Self::Database) -> Expr {
        database.index_metadata(self).expression().clone()
    }
}
