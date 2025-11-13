//! Implement the [`CheckConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{CheckConstraint, CreateTable, Expr};

use crate::{
    structs::{ParserDB, TableAttribute, metadata::CheckMetadata},
    traits::{CheckConstraintLike, DatabaseLike, Metadata},
};

impl Metadata for TableAttribute<CreateTable, CheckConstraint> {
    type Meta = CheckMetadata<Self>;
}

impl CheckConstraintLike for TableAttribute<CreateTable, CheckConstraint> {
    type DB = ParserDB;

    #[inline]
    fn expression<'db>(&'db self, _database: &'db Self::DB) -> &'db Expr {
        self.attribute().expr.as_ref()
    }

    #[inline]
    fn table<'db>(&'db self, database: &'db Self::DB) -> &'db <Self::DB as DatabaseLike>::Table {
        database
            .check_constraint_metadata(self)
            .expect("Check constraint must exist in database")
            .table()
    }

    #[inline]
    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as crate::prelude::DatabaseLike>::Column> {
        database
            .check_constraint_metadata(self)
            .expect("Check constraint must exist in database")
            .columns()
    }

    #[inline]
    fn functions<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as crate::prelude::DatabaseLike>::Function> {
        database
            .check_constraint_metadata(self)
            .expect("Check constraint must exist in database")
            .functions()
    }
}
