//! PostgreSQL data type casts catalog model.
//!
//! This module provides the `PgCast` struct for working with the
//! `pg_catalog.pg_cast` system catalog table.

/// Represents a row from the `pg_catalog.pg_cast` table.
///
/// Contains information about data type conversion functions.
/// This table defines how PostgreSQL can convert values from one data type
/// to another, including the contexts where such conversions are allowed.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_cast::pg_cast)]
pub struct PgCast {
    /// OID of this cast (primary key).
    pub oid: u32,
    /// OID of the source data type.
    pub castsource: u32,
    /// OID of the target data type.
    pub casttarget: u32,
    /// OID of the cast function (0 if no function needed).
    pub castfunc: u32,
    /// Context in which cast can be invoked (e=explicit, a=assignment,
    /// i=implicit).
    pub castcontext: String,
    /// Method of cast (f=function, i=inout, b=binary coercible).
    pub castmethod: String,
}
