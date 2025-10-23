//! Submodule providing the `PgStatUserIndex` struct representing a row of the
//! `pg_stat_user_indexes` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_user_indexes` view.
///
/// The `pg_stat_user_indexes` view shows statistics about accesses to
/// user-defined indexes.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-ALL-INDEXES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_user_indexes::pg_stat_user_indexes)]
pub struct PgStatUserIndex {
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
    /// Index scans count.
    pub idx_scan: Option<i64>,
    /// Last index scan time.
    pub last_idx_scan: Option<SystemTime>,
    /// Index entries read.
    pub idx_tup_read: Option<i64>,
    /// Tuples fetched.
    pub idx_tup_fetch: Option<i64>,
}
