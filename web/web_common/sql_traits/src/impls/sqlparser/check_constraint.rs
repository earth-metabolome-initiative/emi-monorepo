//! Implement the [`CheckConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{CheckConstraint, CreateTable, Expr};

use crate::{
    structs::{ParserDB, TableAttribute, metadata::CheckMetadata},
    traits::{CheckConstraintLike, Metadata},
};

impl Metadata for TableAttribute<CreateTable, CheckConstraint> {
    type Meta = CheckMetadata<Self>;
}

impl CheckConstraintLike for TableAttribute<CreateTable, CheckConstraint> {
    type DB = ParserDB;

    fn expression<'db>(&'db self, _database: &'db Self::DB) -> &'db Expr {
        self.attribute().expr.as_ref()
    }

    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as crate::prelude::DatabaseLike>::Column> {
        database.check_constraint_metadata(self).columns()
    }
}
