//! Submodule providing the `PgStatCheckpointer` struct representing a row of
//! the `pg_stat_checkpointer` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_checkpointer` view.
///
/// The `pg_stat_checkpointer` view shows statistics about the checkpointer
/// process's activity. It contains only one row with cluster-wide data.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-CHECKPOINTER-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_checkpointer::pg_stat_checkpointer)]
pub struct PgStatCheckpointer {
    /// Timed checkpoints.
    pub num_timed: Option<i64>,
    /// Requested checkpoints.
    pub num_requested: Option<i64>,
    /// Timed restartpoints.
    pub restartpoints_timed: Option<i64>,
    /// Requested restartpoints.
    pub restartpoints_req: Option<i64>,
    /// Restartpoints done.
    pub restartpoints_done: Option<i64>,
    /// Write time (milliseconds).
    pub write_time: Option<f64>,
    /// Sync time (milliseconds).
    pub sync_time: Option<f64>,
    /// Buffers written.
    pub buffers_written: Option<i64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
