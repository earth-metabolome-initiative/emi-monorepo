//! Submodule implementing the [`TableLike`] trait for `sqlparser`'s
//! [`CreateTable`](sqlparser::ast::CreateTable) struct.

use ::sqlparser::ast::{CreateTable, Ident};

use crate::{
    structs::{TableMetadata, generic_db::ParserDB},
    traits::{DatabaseLike, Metadata, TableLike},
    utils::last_str,
};

impl Metadata for CreateTable {
    type Meta = TableMetadata<CreateTable>;
}

impl TableLike for CreateTable {
    type DB = ParserDB;

    #[inline]
    fn table_name(&self) -> &str {
        last_str(&self.name)
    }

    #[inline]
    fn table_doc<'db>(&'db self, _database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db,
    {
        // TODO(@RPG-Alex): Extract documentation from SQL comments after merging PR <https://github.com/apache/datafusion-sqlparser-rs/pull/2069>
        Some("Undocumented table")
    }

    #[inline]
    fn table_schema(&self) -> Option<&str> {
        let object_name_parts = &self.name.0;
        if object_name_parts.len() > 1 {
            let schema_part = &object_name_parts[0];
            match schema_part {
                sqlparser::ast::ObjectNamePart::Identifier(Ident { value, .. }) => {
                    Some(value.as_str())
                }
                sqlparser::ast::ObjectNamePart::Function(_) => {
                    panic!("Unexpected object name part in CreateTable: {schema_part:?}")
                }
            }
        } else {
            None
        }
    }

    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).expect("Table must exist in database").columns()
    }

    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        database.table_metadata(self).expect("Table must exist in database").primary_key_columns()
    }

    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::UniqueIndex>
    where
        Self: 'db,
    {
        database.table_metadata(self).expect("Table must exist in database").unique_indices()
    }

    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint>
    where
        Self: 'db,
    {
        database.table_metadata(self).expect("Table must exist in database").check_constraints()
    }

    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        database.table_metadata(self).expect("Table must exist in database").foreign_keys()
    }
}
