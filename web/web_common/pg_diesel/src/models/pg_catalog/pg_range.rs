//! Submodule providing the `PgRange` struct representing a row of the
//! `pg_range` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_range` table.
///
/// The `pg_range` catalog stores information about range types. Range types are
/// data types representing a range of values of some element type (the
/// subtype).
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-range.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_range::pg_range)]
pub struct PgRange {
    /// OID of the range type.
    pub rngtypid: u32,
    /// OID of the subtype.
    pub rngsubtype: u32,
    /// OID of the multirange type.
    pub rngmultitypid: u32,
    /// OID of the collation.
    pub rngcollation: u32,
    /// OID of the subtype's operator class.
    pub rngsubopc: u32,
    /// OID of the canonicalization function.
    pub rngcanonical: u32,
    /// OID of the subtype difference function.
    pub rngsubdiff: u32,
}
