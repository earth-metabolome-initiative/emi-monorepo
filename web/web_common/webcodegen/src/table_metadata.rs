//! Submodule defining diesel-based structs for Postgres core metadata tables.

use diesel::{Queryable, QueryableByName, RunQueryDsl};
use diesel::pg::PgConnection;
use crate::schema::*;

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = tables)]
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
}