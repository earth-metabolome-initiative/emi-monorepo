//! Submodule implementing the [`ColumnLike`] trait for `sqlparser`'s
//! [`ColumnDef`](sqlparser::ast::ColumnDef) struct.

use sqlparser::ast::{ColumnDef, CreateTable, ForeignKeyConstraint};

use crate::{impls::SqlParserDatabase, traits::ColumnLike};

const GENERATIVE_TYPES: &[&str] = &["SERIAL", "BIGSERIAL", "SMALLSERIAL"];
const NORMALIZED_TYPES: &[(&str, &str)] =
    &[("SERIAL", "INT"), ("INTEGER", "INT"), ("BIGSERIAL", "BIGINT"), ("SMALLSERIAL", "SMALLINT")];

impl ColumnLike for ColumnDef {
    type ForeignKey = ForeignKeyConstraint;
    type Database = SqlParserDatabase;
    type Table = CreateTable;

    fn column_name(&self) -> &str {
        self.name.value.as_str()
    }

    fn data_type(&self) -> String {
        self.data_type.to_string()
    }

    fn is_generative(&self) -> bool {
        GENERATIVE_TYPES.contains(&self.data_type.to_string().as_str())
    }

    fn normalized_data_type(&self) -> String {
        let data_type = self.data_type.to_string().to_uppercase();
        for (ty, normalized) in NORMALIZED_TYPES {
            if data_type == *ty {
                return normalized.to_string();
            }
        }
        data_type
    }

    fn is_nullable(&self) -> bool {
        !self.options.iter().any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull))
    }
}
