//! Submodule providing the `PgConversion` struct representing a row of the
//! `pg_conversion` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_conversion` table.
///
/// The `pg_conversion` system catalog stores encoding conversion information.
/// Each row describes a conversion from one character encoding to another,
/// including the function used to perform the conversion and whether it is
/// the default conversion for that encoding pair.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-conversion.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_conversion::pg_conversion)]
pub struct PgConversion {
    /// OID of the conversion.
    pub oid: u32,
    /// Name of the conversion.
    pub conname: String,
    /// OID of the namespace (schema) containing this conversion.
    pub connamespace: u32,
    /// OID of the role that owns the conversion.
    pub conowner: u32,
    /// Source encoding ID.
    pub conforencoding: i32,
    /// Destination encoding ID.
    pub contoencoding: i32,
    /// OID of the conversion function.
    pub conproc: u32,
    /// Whether this is the default conversion for this encoding pair.
    pub condefault: bool,
}
