//! Model for pg_catalog.pg_user_mappings view.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_user_mappings::pg_user_mappings)]
#[diesel(primary_key(umid))]
/// Represents a row from the `pg_user_mappings` view.
pub struct PgUserMappings {
    /// OID of the user mapping
    pub umid: Option<u32>,
    /// OID of the foreign server
    pub srvid: Option<u32>,
    /// Name of the foreign server
    pub srvname: Option<String>,
    /// OID of the local user being mapped, or 0 if the mapping is public
    pub umuser: Option<u32>,
    /// Name of the local user
    pub usename: Option<String>,
    /// User mapping specific options
    pub umoptions: Option<Vec<String>>,
}
