//! Submodule defining diesel-based structs for Postgres core metadata tables.

use diesel::Queryable;

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable)]
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