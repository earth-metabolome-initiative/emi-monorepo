//! Implementation of [`CheckConstraintLike`] for [`CheckConstraint`].
//!
//! This module implements the
//! [`CheckConstraintLike`](sql_traits::prelude::CheckConstraintLike)
//! trait for the [`CheckConstraint`] model, enabling generic introspection of
//! check constraints.
//!
//! The implementation parses the check constraint expression from the
//! `check_clause` field using the PostgreSQL SQL parser.

use sql_traits::{
    structs::metadata::CheckMetadata,
    traits::{CheckConstraintLike, Metadata},
};

use crate::{PgDatabase, models::CheckConstraint};

impl Metadata for CheckConstraint {
    type Meta = CheckMetadata<Self>;
}

impl CheckConstraintLike for CheckConstraint {
    type DB = PgDatabase;

    fn expression<'db>(&'db self, database: &'db Self::DB) -> &'db sqlparser::ast::Expr {
        database.check_constraint_metadata(self).expression()
    }

    fn table<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as sql_traits::prelude::DatabaseLike>::Table {
        database.check_constraint_metadata(self).table()
    }

    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as sql_traits::prelude::DatabaseLike>::Column> {
        database.check_constraint_metadata(self).columns()
    }

    fn functions<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as sql_traits::prelude::DatabaseLike>::Function> + 'db
    {
        database.check_constraint_metadata(self).functions()
    }
}
