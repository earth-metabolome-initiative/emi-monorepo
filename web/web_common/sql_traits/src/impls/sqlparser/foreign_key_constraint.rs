//! Implement the [`ForeignKeyConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{ConstraintReferenceMatchKind, CreateTable, ForeignKeyConstraint};

use crate::{
    structs::{TableAttribute, generic_db::ParserDB},
    traits::{ForeignKeyLike, Metadata, database::DatabaseLike, table::TableLike},
};

impl Metadata for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type Meta = ();
}

impl ForeignKeyLike for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type DB = ParserDB;

    fn foreign_key_name(&self) -> Option<&str> {
        self.attribute().name.as_ref().map(|s| s.value.as_str())
    }

    fn host_table<'db>(
        &'db self,
        _database: &'db Self::DB,
    ) -> &'db <Self::DB as DatabaseLike>::Table
    where
        Self: 'db,
    {
        self.table()
    }

    fn referenced_table<'db>(
        &self,
        database: &'db Self::DB,
    ) -> &'db <Self::DB as DatabaseLike>::Table {
        let referenced_table_name = &self
            .attribute()
            .foreign_table
            .0
            .last()
            .expect("Foreign table name is empty")
            .to_string();
        database
            .tables()
            .find(|table: &&<Self::DB as DatabaseLike>::Table| {
                table.table_name() == referenced_table_name
            })
            .expect(&format!(
                "Referenced table `{referenced_table_name}` not found for foreign key"
            ))
    }

    fn on_delete_cascade(&self, _database: &Self::DB) -> bool {
        matches!(self.attribute().on_delete, Some(sqlparser::ast::ReferentialAction::Cascade))
    }

    fn match_kind(&self, _database: &Self::DB) -> ConstraintReferenceMatchKind {
        self.attribute().match_kind.clone().unwrap_or(ConstraintReferenceMatchKind::Simple)
    }

    fn host_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let host_table = self.host_table(database);
        self.attribute().columns.iter().filter_map(move |col_name| {
            host_table
                .columns(database)
                .find(|col: &&<Self::DB as DatabaseLike>::Column| &col.attribute().name == col_name)
        })
    }

    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let referenced_table = self.referenced_table(database);
        self.attribute().referred_columns.iter().filter_map(move |col_name| {
            referenced_table
                .columns(database)
                .find(|col: &&<Self::DB as DatabaseLike>::Column| &col.attribute().name == col_name)
        })
    }
}
