//! Model for pg_catalog.pg_ts_dict table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ts_dict::pg_ts_dict)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_ts_dict` catalog table.
pub struct PgTsDict {
    /// Row identifier
    pub oid: u32,
    /// Text search dictionary name
    pub dictname: String,
    /// The OID of the namespace that contains this dictionary
    pub dictnamespace: u32,
    /// Owner of the dictionary
    pub dictowner: u32,
    /// OID of the text search template for this dictionary
    pub dicttemplate: u32,
    /// Initialization option string for the dictionary
    pub dictinitoption: Option<String>,
}
