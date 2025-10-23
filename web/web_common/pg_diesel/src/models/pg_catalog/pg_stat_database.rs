//! Submodule providing the `PgStatDatabase` struct representing a row of the
//! `pg_stat_database` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_database` view.
///
/// The `pg_stat_database` view shows one row per database showing statistics
/// about accesses to that database.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-DATABASE-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_database::pg_stat_database)]
pub struct PgStatDatabase {
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Number of backends.
    pub numbackends: Option<i32>,
    /// Committed transactions.
    pub xact_commit: Option<i64>,
    /// Rolled back transactions.
    pub xact_rollback: Option<i64>,
    /// Blocks read from disk.
    pub blks_read: Option<i64>,
    /// Buffer hits.
    pub blks_hit: Option<i64>,
    /// Tuples returned.
    pub tup_returned: Option<i64>,
    /// Tuples fetched.
    pub tup_fetched: Option<i64>,
    /// Tuples inserted.
    pub tup_inserted: Option<i64>,
    /// Tuples updated.
    pub tup_updated: Option<i64>,
    /// Tuples deleted.
    pub tup_deleted: Option<i64>,
    /// Conflicts.
    pub conflicts: Option<i64>,
    /// Temporary files.
    pub temp_files: Option<i64>,
    /// Temporary bytes.
    pub temp_bytes: Option<i64>,
    /// Deadlocks.
    pub deadlocks: Option<i64>,
    /// Checksum failures.
    pub checksum_failures: Option<i64>,
    /// Last checksum failure time.
    pub checksum_last_failure: Option<SystemTime>,
    /// Block read time (milliseconds).
    pub blk_read_time: Option<f64>,
    /// Block write time (milliseconds).
    pub blk_write_time: Option<f64>,
    /// Session time (milliseconds).
    pub session_time: Option<f64>,
    /// Active time (milliseconds).
    pub active_time: Option<f64>,
    /// Idle in transaction time (milliseconds).
    pub idle_in_transaction_time: Option<f64>,
    /// Total sessions.
    pub sessions: Option<i64>,
    /// Abandoned sessions.
    pub sessions_abandoned: Option<i64>,
    /// Fatal sessions.
    pub sessions_fatal: Option<i64>,
    /// Killed sessions.
    pub sessions_killed: Option<i64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
