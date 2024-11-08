use diesel::pg::PgConnection;
use diesel::{Queryable, QueryableByName, RunQueryDsl, Selectable};

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Selectable, Debug)]
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
    pub is_updatable: String,
}

impl Column {
    pub fn load_all_columns(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::columns::dsl::*;
        columns.load::<Column>(conn).expect("Error loading columns")
    }
}
