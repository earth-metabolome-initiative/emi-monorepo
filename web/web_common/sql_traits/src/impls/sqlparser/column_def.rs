//! Submodule implementing the [`ColumnLike`] trait for `sqlparser`'s
//! [`ColumnDef`](sqlparser::ast::ColumnDef) struct.

use sqlparser::ast::{ColumnDef, CreateTable};

use crate::{
    structs::{generic_db::ParserDB, metadata::TableAttribute},
    traits::{ColumnLike, DatabaseLike, Metadata},
};

const GENERATED_TYPES: &[&str] = &["SERIAL", "BIGSERIAL", "SMALLSERIAL"];
const NORMALIZED_TYPES: &[(&str, &str)] =
    &[("SERIAL", "INT"), ("INTEGER", "INT"), ("BIGSERIAL", "BIGINT"), ("SMALLSERIAL", "SMALLINT")];

impl Metadata for TableAttribute<CreateTable, ColumnDef> {
    type Meta = ();
}

impl ColumnLike for TableAttribute<CreateTable, ColumnDef> {
    type DB = ParserDB;

    fn column_name(&self) -> &str {
        self.attribute().name.value.as_str()
    }

    fn column_doc<'db>(&'db self, _database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db,
    {
        // TODO(@RPG-Alex): Extract documentation from SQL comments after merging PR <https://github.com/apache/datafusion-sqlparser-rs/pull/2069>
        Some("Undocumented column")
    }

    fn data_type(&self) -> String {
        self.attribute().data_type.to_string()
    }

    fn is_generated(&self) -> bool {
        GENERATED_TYPES.contains(&self.attribute().data_type.to_string().as_str())
    }

    fn normalized_data_type(&self, _database: &Self::DB) -> String {
        let data_type = self.attribute().data_type.to_string().to_uppercase();
        for (ty, normalized) in NORMALIZED_TYPES {
            if data_type == *ty {
                return normalized.to_string();
            }
        }
        data_type
    }

    fn is_nullable(&self) -> bool {
        !self
            .attribute()
            .options
            .iter()
            .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull))
    }

    fn has_default(&self) -> bool {
        self.attribute()
            .options
            .iter()
            .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::Default(_)))
    }

    fn table<'a>(&'a self, _database: &'a Self::DB) -> &'a <Self::DB as DatabaseLike>::Table
    where
        Self: 'a,
    {
        self.table()
    }
}
