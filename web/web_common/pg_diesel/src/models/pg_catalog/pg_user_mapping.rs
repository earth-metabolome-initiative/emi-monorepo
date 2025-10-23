//! Model for pg_catalog.pg_user_mapping table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_user_mapping::pg_user_mapping)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_user_mapping` catalog table.
pub struct PgUserMapping {
    /// Row identifier
    pub oid: u32,
    /// OID of the local user being mapped, or 0 if the mapping is public
    pub umuser: u32,
    /// OID of the foreign server that contains this mapping
    pub umserver: u32,
    /// User mapping specific options, as "keyword=value" strings
    pub umoptions: Option<Vec<String>>,
}
