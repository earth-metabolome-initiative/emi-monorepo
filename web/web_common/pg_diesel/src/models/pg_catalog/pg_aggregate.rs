//! PostgreSQL aggregate functions catalog model.
//!
//! This module provides the `PgAggregate` struct for working with the
//! `pg_catalog.pg_aggregate` system catalog table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_catalog.pg_aggregate` table.
///
/// Contains information about aggregate functions including their transition
/// functions, final functions, and other properties specific to aggregation.
/// This table stores metadata about how PostgreSQL aggregate functions work
/// internally.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_aggregate::pg_aggregate)]
pub struct PgAggregate {
    /// OID of the aggregate function.
    pub aggfnoid: u32,
    /// Kind of aggregate function (normal, ordered-set, hypothetical-set).
    pub aggkind: String,
    /// Number of direct (non-aggregated) arguments.
    pub aggnumdirectargs: i16,
    /// Transition function OID.
    pub aggtransfn: u32,
    /// Final function OID.
    pub aggfinalfn: u32,
    /// Combine function OID for parallel aggregation.
    pub aggcombinefn: u32,
    /// Serial function OID for parallel aggregation.
    pub aggserialfn: u32,
    /// Deserial function OID for parallel aggregation.
    pub aggdeserialfn: u32,
    /// Moving-aggregate transition function OID.
    pub aggmtransfn: u32,
    /// Moving-aggregate inverse transition function OID.
    pub aggminvtransfn: u32,
    /// Moving-aggregate final function OID.
    pub aggmfinalfn: u32,
    /// Whether final function wants extra dummy arguments.
    pub aggfinalextra: bool,
    /// Whether moving-aggregate final function wants extra dummy arguments.
    pub aggmfinalextra: bool,
    /// Whether final function modifies transition state.
    pub aggfinalmodify: String,
    /// Whether moving-aggregate final function modifies transition state.
    pub aggmfinalmodify: String,
    /// Associated sort operator OID.
    pub aggsortop: u32,
    /// Data type of transition state.
    pub aggtranstype: u32,
    /// Approximate average size of transition state.
    pub aggtransspace: i32,
    /// Data type of moving-aggregate transition state.
    pub aggmtranstype: u32,
    /// Approximate average size of moving-aggregate transition state.
    pub aggmtransspace: i32,
    /// Initial value of transition state.
    pub agginitval: Option<String>,
    /// Initial value of moving-aggregate transition state.
    pub aggminitval: Option<String>,
}
