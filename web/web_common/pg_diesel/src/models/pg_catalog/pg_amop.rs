//! PostgreSQL access method operators catalog model.
//!
//! This module provides the `PgAmop` struct for working with the
//! `pg_catalog.pg_amop` system catalog table.

/// Represents a row from the `pg_catalog.pg_amop` table.
///
/// Contains information about operators that can be used with particular access
/// method operator families. This table defines which operators are supported
/// by various index access methods.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_amop::pg_amop)]
pub struct PgAmop {
    /// OID of this entry.
    pub oid: u32,
    /// OID of operator family this entry belongs to.
    pub amopfamily: u32,
    /// Left-hand input data type of operator.
    pub amoplefttype: u32,
    /// Right-hand input data type of operator.
    pub amoprighttype: u32,
    /// Strategy number associated with operator.
    pub amopstrategy: i16,
    /// Purpose of operator (s=search, o=ordering).
    pub amoppurpose: String,
    /// OID of the operator.
    pub amopopr: u32,
    /// OID of access method this operator belongs to.
    pub amopmethod: u32,
    /// OID of sort family, if ordering operator.
    pub amopsortfamily: u32,
}
