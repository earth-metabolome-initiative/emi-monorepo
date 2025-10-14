//! Implement the [`UniqueConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{ColumnDef, CreateTable, Expr, UniqueConstraint};

use crate::{
    structs::{TableAttribute, generic_db::ParserDB, metadata::UniqueIndexMetadata},
    traits::{Metadata, UniqueIndexLike},
};

impl Metadata for TableAttribute<CreateTable, UniqueConstraint> {
    type Meta = UniqueIndexMetadata<Self>;
}

impl UniqueIndexLike for TableAttribute<CreateTable, UniqueConstraint> {
    type Table = CreateTable;
    type Database = ParserDB;
    type Column = TableAttribute<CreateTable, ColumnDef>;

    fn table<'db>(&'db self, _database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db,
    {
        self.table()
    }

    fn expression<'db>(&'db self, database: &'db Self::Database) -> &'db Expr
    where
        Self: 'db,
    {
        database.index_metadata(self).expression()
    }
}
