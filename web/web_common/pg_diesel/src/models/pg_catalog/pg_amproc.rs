//! PostgreSQL access method support procedures catalog model.
//!
//! This module provides the `PgAmproc` struct for working with the
//! `pg_catalog.pg_amproc` system catalog table.

/// Represents a row from the `pg_catalog.pg_amproc` table.
///
/// Contains information about support procedures used by access method operator
/// families. These procedures provide the functionality needed by index access
/// methods to operate.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_amproc::pg_amproc)]
pub struct PgAmproc {
    /// OID of this entry.
    pub oid: u32,
    /// OID of operator family this procedure belongs to.
    pub amprocfamily: u32,
    /// Left-hand input data type, or zero if not type-specific.
    pub amproclefttype: u32,
    /// Right-hand input data type, or zero if not type-specific.
    pub amprocrighttype: u32,
    /// Support procedure number.
    pub amprocnum: i16,
    /// OID of the procedure.
    pub amproc: u32,
}
