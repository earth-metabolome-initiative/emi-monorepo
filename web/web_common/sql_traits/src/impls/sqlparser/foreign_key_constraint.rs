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

    fn foreign_key_name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.value.as_str())
    }

    fn referenced_table<'db>(&self, database: &'db Self::Database) -> &'db Self::Table {
        let referenced_table_name =
            &self.foreign_table.0.last().expect("Foreign table name is empty").to_string();
        database.tables().find(|table| table.table_name() == referenced_table_name).expect(
            &format!("Referenced table `{referenced_table_name}` not found for foreign key"),
        )
    }

    fn host_columns(
        &self,
        database: &Self::Database,
        host_table: &Self::Table,
    ) -> impl Iterator<Item = Self::Column> {
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
