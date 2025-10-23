//! Model for pg_catalog.pg_ts_config table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ts_config::pg_ts_config)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_ts_config` catalog table.
pub struct PgTsConfig {
    /// Row identifier
    pub oid: u32,
    /// Text search configuration name
    pub cfgname: String,
    /// The OID of the namespace that contains this configuration
    pub cfgnamespace: u32,
    /// Owner of the configuration
    pub cfgowner: u32,
    /// The OID of the text search parser for this configuration
    pub cfgparser: u32,
}
