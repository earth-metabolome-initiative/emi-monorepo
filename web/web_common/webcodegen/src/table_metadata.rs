//! Submodule defining diesel-based structs for Postgres core metadata tables.

use diesel::{Queryable, QueryDsl, QueryableByName, RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::tables)]
pub struct Table {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: Option<String>,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: Option<String>,
}

impl Table {
    pub fn load_all_tables(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::tables::dsl::*;
        tables
            .load::<Table>(conn)
            .expect("Error loading tables")
    }

    pub fn load(conn: &mut PgConnection, table_name: &str, table_schema: Option<&str>, table_catalog: &str) -> Option<Self> {
        use crate::schema::tables;
        let table_schema = table_schema.unwrap_or("public");
        tables::dsl::tables
            .filter(tables::dsl::table_name.eq(table_name))
            .filter(tables::dsl::table_schema.eq(table_schema))
            .filter(tables::dsl::table_catalog.eq(table_catalog))
            .first::<Table>(conn)
            .ok()
    }

    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, DieselError> {
        use crate::schema::columns;
        columns::dsl::columns
            .filter(columns::dsl::table_name.eq(&self.table_name))
            .filter(columns::dsl::table_schema.eq(&self.table_schema))
            .filter(columns::dsl::table_catalog.eq(&self.table_catalog))
            .load::<Column>(conn)
    }
}

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::columns)]
pub struct Column {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: i32,
    pub column_default: Option<String>,
    pub __is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub character_octet_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_precision_radix: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub datetime_precision: Option<i32>,
    pub interval_type: Option<String>,
    pub interval_precision: Option<i32>,
    pub character_set_catalog: Option<String>,
    pub character_set_schema: Option<String>,
    pub character_set_name: Option<String>,
    pub collation_catalog: Option<String>,
    pub collation_schema: Option<String>,
    pub collation_name: Option<String>,
    pub domain_catalog: Option<String>,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub udt_catalog: Option<String>,
    pub udt_schema: Option<String>,
    pub udt_name: Option<String>,
    pub scope_catalog: Option<String>,
    pub scope_schema: Option<String>,
    pub scope_name: Option<String>,
    pub maximum_cardinality: Option<i32>,
    pub dtd_identifier: Option<String>,
    pub is_self_referencing: Option<String>,
    pub is_identity: Option<String>,
    pub identity_generation: Option<String>,
    pub identity_start: Option<String>,
    pub identity_increment: Option<String>,
    pub identity_maximum: Option<String>,
    pub identity_minimum: Option<String>,
    pub identity_cycle: Option<String>,
    pub is_generated: String,
    pub generation_expression: Option<String>,
    pub is_updatable: String
}
