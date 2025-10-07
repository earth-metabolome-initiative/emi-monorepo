//! Implement the [`ForeignKeyConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{ColumnDef, CreateTable, ForeignKeyConstraint};

use crate::{
    impls::SqlParserDatabase,
    traits::{ForeignKeyLike, database::DatabaseLike, table::TableLike},
};

impl ForeignKeyLike for ForeignKeyConstraint {
    type Column = ColumnDef;
    type Table = CreateTable;
    type Database = SqlParserDatabase;

    fn host_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        database
            .tables()
            .find(|table| table.foreign_keys(database).any(|fk| &fk == self))
            .expect("Host table not found for foreign key")
    }

    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        let referenced_table_name =
            &self.foreign_table.0.last().expect("Foreign table name is empty").to_string();
        database.tables().find(|table| table.table_name() == referenced_table_name).expect(
            &format!("Referenced table `{referenced_table_name}` not found for foreign key"),
        )
    }

    fn host_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        let host_table = self.host_table(database);
        self.columns.iter().filter_map(move |col_name| {
            host_table.columns(database).find(|col| &col.name == col_name)
        })
    }

    fn referenced_columns(&self, database: &Self::Database) -> impl Iterator<Item = Self::Column> {
        let referenced_table = self.referenced_table(database);
        self.referred_columns.iter().filter_map(move |col_name| {
            referenced_table.columns(database).find(|col| &col.name == col_name)
        })
    }
}
