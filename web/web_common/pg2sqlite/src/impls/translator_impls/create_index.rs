//! Implementation of the [`Translator`] trait for the
//! [`CreateIndex`](sqlparser::ast::CreateIndex) type.

use sqlparser::{
    ast::{
        CreateIndex, CreateTable, DataType, Expr, Ident, IndexType, ObjectName, ObjectNamePart,
        SqlOption, Statement, Value, ValueWithSpan,
    },
    tokenizer::{Location, Span},
};

use crate::prelude::{Pg2Sqlite, Translator};

fn create_fts5_from_index(create_index: &CreateIndex) -> CreateTable {
    let table_name = create_index.table_name.to_string();
    CreateTable {
        or_replace: false,
        temporary: false,
        external: false,
        global: None,
        if_not_exists: false,
        transient: false,
        volatile: false,
        iceberg: false,
        name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new(format!(
            "{table_name}_fts5",
        )))]),
        columns: vec![
            // FTS5 column definition
            sqlparser::ast::ColumnDef {
                name: Ident::new("name"),
                data_type: DataType::Text,
                options: vec![],
            },
        ],
        constraints: vec![],
        hive_formats: None,
        table_properties: vec![
            SqlOption::KeyValue {
                key: Ident::new("using"),
                value: Expr::Value(ValueWithSpan {
                    value: Value::SingleQuotedString("fts5".to_owned()),
                    span: Span {
                        start: Location { line: 0, column: 0 },
                        end: Location { line: 0, column: 0 },
                    },
                }),
            },
            SqlOption::KeyValue {
                key: Ident::new("content"),
                value: Expr::Value(ValueWithSpan {
                    value: Value::SingleQuotedString(table_name),
                    span: Span {
                        start: Location { line: 0, column: 0 },
                        end: Location { line: 0, column: 0 },
                    },
                }),
            },
            SqlOption::KeyValue {
                key: Ident::new("content_rowid"),
                value: Expr::Value(ValueWithSpan {
                    value: Value::SingleQuotedString("rowid".to_owned()),
                    span: Span {
                        start: Location { line: 0, column: 0 },
                        end: Location { line: 0, column: 0 },
                    },
                }),
            },
        ],
        with_options: vec![],
        file_format: None,
        location: None,
        query: None,
        without_rowid: false,
        like: None,
        clone: None,
        engine: None,
        default_charset: None,
        comment: None,
        collation: None,
        on_commit: None,
        on_cluster: None,
        storage_serialization_policy: None,
        auto_increment_offset: None,
        catalog_sync: None,
        catalog: None,
        base_location: None,
        external_volume: None,
        with_tags: None,
        with_row_access_policy: None,
        with_aggregation_policy: None,
        default_ddl_collation: None,
        max_data_extension_time_in_days: None,
        data_retention_time_in_days: None,
        change_tracking: None,
        enable_schema_evolution: None,
        strict: true,
        copy_grants: false,
        cluster_by: None,
        hive_distribution: sqlparser::ast::HiveDistributionStyle::NONE,
        options: None,
        partition_by: None,
        order_by: None,
        clustered_by: None,
        primary_key: None,
    }
}

impl Translator for CreateIndex {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = Vec<Statement>;

    fn translate(&self, _schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        // If the index is a GIN or GiST index, we need to translate it into a table
        // with a FTS5 virtual table. This is because SQLite does not support
        // GIN or GiST indexes.
        if let Some(IndexType::GIN) | Some(IndexType::GiST) = self.using {
            let _fts5_table = create_fts5_from_index(self);
        }

        unimplemented!("CreateIndex translation for definition `{}` is not yet implemented.", self)
    }
}
