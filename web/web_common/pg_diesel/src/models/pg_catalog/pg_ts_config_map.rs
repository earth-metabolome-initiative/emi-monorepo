//! Model for pg_catalog.pg_ts_config_map table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ts_config_map::pg_ts_config_map)]
#[diesel(primary_key(mapcfg, maptokentype, mapseqno))]
/// Represents a row from the `pg_ts_config_map` catalog table.
pub struct PgTsConfigMap {
    /// OID of the `pg_ts_config` entry owning this map entry
    pub mapcfg: u32,
    /// Token type that this entry is for
    pub maptokentype: i32,
    /// Order in which to consult this entry (lower numbers first)
    pub mapseqno: i32,
    /// OID of the text search dictionary to consult
    pub mapdict: u32,
}
