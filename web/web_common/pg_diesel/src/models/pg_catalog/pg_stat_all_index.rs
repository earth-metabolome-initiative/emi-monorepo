//! Submodule providing the `PgStatAllIndex` struct representing a row of the
//! `pg_stat_all_indexes` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_all_indexes` view.
///
/// The `pg_stat_all_indexes` view shows statistics about accesses to each
/// index in the database.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-ALL-INDEXES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_all_indexes::pg_stat_all_indexes)]
pub struct PgStatAllIndex {
    /// Table OID.
    pub relid: Option<u32>,
    /// Index OID.
    pub indexrelid: Option<u32>,
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub relname: Option<String>,
    /// Index name.
    pub indexrelname: Option<String>,
    /// Number of scans.
    pub idx_scan: Option<i64>,
    /// Last scan time.
    pub last_idx_scan: Option<SystemTime>,
    /// Tuples read.
    pub idx_tup_read: Option<i64>,
    /// Tuples fetched.
    pub idx_tup_fetch: Option<i64>,
}
