//! Implement the [`ForeignKeyConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{ColumnDef, CreateTable, ForeignKeyConstraint, MatchKind, UniqueConstraint};

use crate::{
    structs::{TableAttribute, generic_db::ParserDB},
    traits::{ForeignKeyLike, Metadata, database::DatabaseLike, table::TableLike},
};

impl Metadata for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type Meta = ();
}

impl ForeignKeyLike for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type Column = TableAttribute<CreateTable, ColumnDef>;
    type Table = CreateTable;
    type Database = ParserDB;
    type UniqueIndex = TableAttribute<CreateTable, UniqueConstraint>;

    fn foreign_key_name(&self) -> Option<&str> {
        self.attribute().name.as_ref().map(|s| s.value.as_str())
    }

    fn host_table<'db>(&'db self, _database: &'db Self::Database) -> &'db Self::Table
    where
        Self: 'db,
    {
        self.table()
    }

    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        let referenced_table_name = &self
            .attribute()
            .foreign_table
            .0
            .last()
            .expect("Foreign table name is empty")
            .to_string();
        database.tables().find(|table| table.table_name() == referenced_table_name).expect(
            &format!("Referenced table `{referenced_table_name}` not found for foreign key"),
        )
    }

    fn on_delete_cascade(&self, _database: &Self::Database) -> bool {
        matches!(self.attribute().on_delete, Some(sqlparser::ast::ReferentialAction::Cascade))
    }

    fn match_kind(&self, _database: &Self::Database) -> MatchKind {
        self.attribute().match_kind.clone().unwrap_or(MatchKind::Simple)
    }

    fn host_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        let host_table = self.host_table(database);
        self.attribute().columns.iter().filter_map(move |col_name| {
            host_table.columns(database).find(|col| &col.attribute().name == col_name)
        })
    }

    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        let referenced_table = self.referenced_table(database);
        self.attribute().referred_columns.iter().filter_map(move |col_name| {
            referenced_table.columns(database).find(|col| &col.attribute().name == col_name)
        })
    }
}
