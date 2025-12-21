//! Submodule implementing the [`ColumnLike`] trait for `sqlparser`'s
//! [`ColumnDef`](sqlparser::ast::ColumnDef) struct.

use sqlparser::ast::{ColumnDef, CreateTable};

use crate::{
    structs::{generic_db::ParserDB, metadata::TableAttribute},
    traits::{ColumnLike, DatabaseLike, Metadata},
    utils::normalize_sqlparser_type,
};

const GENERATED_TYPES: &[&str] = &["SERIAL", "BIGSERIAL", "SMALLSERIAL"];

impl Metadata for TableAttribute<CreateTable, ColumnDef> {
    type Meta = ();
}

impl ColumnLike for TableAttribute<CreateTable, ColumnDef> {
    type DB = ParserDB;

    #[inline]
    fn column_name(&self) -> &str {
        self.attribute().name.value.as_str()
    }

    #[inline]
    fn column_doc<'db>(&'db self, _database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db,
    {
        // TODO(@RPG-Alex): Extract documentation from SQL comments after merging PR <https://github.com/apache/datafusion-sqlparser-rs/pull/2069>
        Some("Undocumented column")
    }

    #[inline]
    fn data_type<'db>(&'db self, _database: &'db Self::DB) -> &'db str {
        normalize_sqlparser_type(&self.attribute().data_type)
    }

    #[inline]
    fn is_generated(&self) -> bool {
        GENERATED_TYPES.contains(&self.attribute().data_type.to_string().as_str())
    }

    #[inline]
    fn is_nullable(&self, database: &Self::DB) -> bool {
        !self
            .attribute()
            .options
            .iter()
            .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull))
            && !self.is_primary_key(database)
    }

    #[inline]
    fn default_value(&self) -> Option<String> {
        self.attribute().options.iter().find_map(|opt| {
            if let sqlparser::ast::ColumnOption::Default(expr) = &opt.option {
                Some(expr.to_string())
            } else {
                None
            }
        })
    }

    #[inline]
    fn table<'a>(&'a self, _database: &'a Self::DB) -> &'a <Self::DB as DatabaseLike>::Table
    where
        Self: 'a,
    {
        self.table()
    }
}
