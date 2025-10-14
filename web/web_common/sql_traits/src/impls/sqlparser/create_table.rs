//! Submodule implementing the [`TableLike`] trait for `sqlparser`'s
//! [`CreateTable`](sqlparser::ast::CreateTable) struct.


use ::sqlparser::ast::{CreateTable, Ident};
use sqlparser::ast::{CheckConstraint, ColumnDef, ForeignKeyConstraint, UniqueConstraint};

use crate::{
    structs::{generic_db::ParserDB, metadata::TableAttribute, TableMetadata},
    traits::{Metadata, TableLike},
};

impl Metadata for CreateTable {
    type Meta = TableMetadata<CreateTable>;
}

impl TableLike for CreateTable {
    type Database = ParserDB;
    type Column = TableAttribute<CreateTable, ColumnDef>;
    type CheckConstraint = TableAttribute<CreateTable, CheckConstraint>;
    type UniqueIndex = TableAttribute<CreateTable, UniqueConstraint>;
    type ForeignKey = TableAttribute<CreateTable, ForeignKeyConstraint>;

    fn table_name(&self) -> &str {
        let object_name_parts = &self.name.0;
        let last_object_name_parts = &object_name_parts[object_name_parts.len() - 1];
        match last_object_name_parts {
            sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => value.as_str(),
            _ => panic!("Unexpected object name part in CreateTable: {:?}", last_object_name_parts),
        }
    }

    fn table_schema(&self) -> Option<&str> {
        let object_name_parts = &self.name.0;
        if object_name_parts.len() > 1 {
            let schema_part = &object_name_parts[0];
            match schema_part {
                sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => {
                    Some(value.as_str())
                }
                _ => panic!("Unexpected object name part in CreateTable: {:?}", schema_part),
            }
        } else {
            None
        }
    }

    fn columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).columns()
    }

    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).primary_key_columns()
    }

    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::UniqueIndex>
    where
        Self: 'db,
    {
        database.table_metadata(self).unique_indices()
    }

    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::CheckConstraint>
    where
        Self: 'db,
    {
        database.table_metadata(self).check_constraints()
    }

    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::ForeignKey>
    where
        Self: 'db,
    {
        database.table_metadata(self).foreign_keys()
    }
}
