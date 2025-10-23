//! PostgreSQL access methods catalog model.
//!
//! This module provides the `PgAm` struct for working with the
//! `pg_catalog.pg_am` system catalog table.

/// Represents a row from the `pg_catalog.pg_am` table.
///
/// Contains information about available index access methods (btree, hash, gin,
/// gist, etc.). Access methods define how PostgreSQL can access and organize
/// data in tables and indexes.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_am::pg_am)]
pub struct PgAm {
    /// OID of the access method.
    pub oid: u32,
    /// Name of the access method.
    pub amname: String,
    /// OID of handler function for this access method.
    pub amhandler: u32,
    /// Type of access method (i=index, t=table).
    pub amtype: String,
}
