//! Submodule providing the `PgStatWal` struct representing a row of the
//! `pg_stat_wal` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_wal` view.
///
/// The `pg_stat_wal` view shows cluster-wide statistics about Write-Ahead Log
/// (WAL) activity.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-WAL-VIEW).
///
/// Note: This struct does not derive `Hash`, `Eq`, or `Ord` because it contains
/// `f64` fields.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_wal::pg_stat_wal)]
pub struct PgStatWal {
    /// WAL records generated.
    pub wal_records: Option<i64>,
    /// Full page images generated.
    pub wal_fpi: Option<i64>,
    /// WAL bytes generated.
    pub wal_bytes: Option<f64>,
    /// Times WAL buffers were full.
    pub wal_buffers_full: Option<i64>,
    /// Number of WAL writes.
    pub wal_write: Option<i64>,
    /// Number of WAL syncs.
    pub wal_sync: Option<i64>,
    /// WAL write time (milliseconds).
    pub wal_write_time: Option<f64>,
    /// WAL sync time (milliseconds).
    pub wal_sync_time: Option<f64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
