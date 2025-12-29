//! Implement the [`ForeignKeyConstraint`] trait for the `sqlparser` crate's

use sqlparser::ast::{ConstraintReferenceMatchKind, CreateTable, ForeignKeyConstraint};

use crate::{
    structs::{TableAttribute, generic_db::ParserDB},
    traits::{ForeignKeyLike, Metadata, database::DatabaseLike, table::TableLike},
    utils::last_str,
};

impl Metadata for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type Meta = ();
}

impl ForeignKeyLike for TableAttribute<CreateTable, ForeignKeyConstraint> {
    type DB = ParserDB;

    #[inline]
    fn foreign_key_name(&self) -> Option<&str> {
        self.attribute().name.as_ref().map(|s| s.value.as_str())
    }

    #[inline]
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
        let referenced_table_name = last_str(&self.attribute().foreign_table);
        database
            .tables()
            .find(|table: &&<Self::DB as DatabaseLike>::Table| {
                table.table_name() == referenced_table_name
            })
            .unwrap_or_else(|| {
                let host_table = self.host_table(database);
                panic!("Referenced table `{referenced_table_name}` not found for foreign key in table `{}`", host_table.table_name())
            })
    }

    #[inline]
    fn on_delete_cascade(&self, _database: &Self::DB) -> bool {
        matches!(self.attribute().on_delete, Some(sqlparser::ast::ReferentialAction::Cascade))
    }

    #[inline]
    fn match_kind(&self, _database: &Self::DB) -> ConstraintReferenceMatchKind {
        self.attribute().match_kind.unwrap_or(ConstraintReferenceMatchKind::Simple)
    }

    fn host_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let host_table = self.host_table(database);
        self.attribute().columns.iter().map(move |col_name| {
            host_table
                .columns(database)
                .find(|col: &&<Self::DB as DatabaseLike>::Column| &col.attribute().name == col_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Host column `{}` not found in table `{}` for foreign key, options: {:?}",
                        col_name,
                        host_table.table_name(),
                        host_table
                            .columns(database)
                            .map(|c| c.attribute().name.to_string())
                            .collect::<Vec<_>>()
                    )
                })
        })
    }

    fn referenced_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let host_table = self.host_table(database);
        let referenced_table = self.referenced_table(database);
        self.attribute().referred_columns.iter().map(move |col_name| {
            referenced_table
                .columns(database)
                .find(|col: &&<Self::DB as DatabaseLike>::Column| &col.attribute().name == col_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Referenced column `{}` in table `{}` not found in table `{}` for foreign key",
                        col_name,
                        host_table.table_name(),
                        referenced_table.table_name()
                    )
                })
        })
    }
}
