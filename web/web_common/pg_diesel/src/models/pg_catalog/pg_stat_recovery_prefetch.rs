//! Submodule providing the `PgStatRecoveryPrefetch` struct representing a row
//! of the `pg_stat_recovery_prefetch` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_recovery_prefetch` view.
///
/// The `pg_stat_recovery_prefetch` view shows statistics about blocks
/// prefetched during recovery. It contains only one row.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-RECOVERY-PREFETCH-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_recovery_prefetch::pg_stat_recovery_prefetch)]
pub struct PgStatRecoveryPrefetch {
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
    /// Blocks prefetched.
    pub prefetch: Option<i64>,
    /// Buffer hits.
    pub hit: Option<i64>,
    /// Skipped: not initialized.
    pub skip_init: Option<i64>,
    /// Skipped: didn't exist.
    pub skip_new: Option<i64>,
    /// Skipped: full page write.
    pub skip_fpw: Option<i64>,
    /// Skipped: recently accessed.
    pub skip_rep: Option<i64>,
    /// WAL distance (bytes).
    pub wal_distance: Option<i32>,
    /// Block distance.
    pub block_distance: Option<i32>,
    /// I/O depth.
    pub io_depth: Option<i32>,
}
