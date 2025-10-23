//! PostgreSQL collation catalog model.
//!
//! This module provides the `PgCollation` struct for working with the
//! `pg_catalog.pg_collation` system catalog table.

/// Represents a row from the `pg_catalog.pg_collation` table.
///
/// Contains information about collations (sorting and character classification
/// rules). Collations define how text strings are sorted and compared in
/// PostgreSQL, supporting multiple languages and cultural conventions.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_collation::pg_collation)]
pub struct PgCollation {
    /// OID of the collation (primary key).
    pub oid: u32,
    /// Name of the collation.
    pub collname: String,
    /// OID of the namespace containing this collation.
    pub collnamespace: u32,
    /// OID of the owner of this collation.
    pub collowner: u32,
    /// Provider of the collation (c=libc, i=icu, d=database default).
    pub collprovider: String,
    /// Whether the collation is deterministic.
    pub collisdeterministic: bool,
    /// Encoding for which this collation is applicable (-1 for any).
    pub collencoding: i32,
    /// LC_COLLATE setting for this collation.
    pub collcollate: Option<String>,
    /// LC_CTYPE setting for this collation.
    pub collctype: Option<String>,
    /// Locale name for ICU collations.
    pub colllocale: Option<String>,
    /// ICU rules for this collation.
    pub collicurules: Option<String>,
    /// Provider-specific version string.
    pub collversion: Option<String>,
}
