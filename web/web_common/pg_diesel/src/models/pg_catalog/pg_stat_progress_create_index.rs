//! Submodule providing the `PgStatProgressCreateIndex` struct representing a
//! row of the `pg_stat_progress_create_index` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_progress_create_index` view.
///
/// The `pg_stat_progress_create_index` view shows progress information for each
/// backend creating an index.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/progress-reporting.html#CREATE-INDEX-PROGRESS-REPORTING).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_progress_create_index::pg_stat_progress_create_index)]
pub struct PgStatProgressCreateIndex {
    /// Process ID.
    pub pid: Option<i32>,
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Table OID.
    pub relid: Option<u32>,
    /// Index OID.
    pub index_relid: Option<u32>,
    /// Command (CREATE INDEX/REINDEX).
    pub command: Option<String>,
    /// Current phase.
    pub phase: Option<String>,
    /// Total lockers.
    pub lockers_total: Option<i64>,
    /// Lockers done.
    pub lockers_done: Option<i64>,
    /// Current locker PID.
    pub current_locker_pid: Option<i64>,
    /// Total blocks.
    pub blocks_total: Option<i64>,
    /// Blocks done.
    pub blocks_done: Option<i64>,
    /// Total tuples.
    pub tuples_total: Option<i64>,
    /// Tuples done.
    pub tuples_done: Option<i64>,
    /// Total partitions.
    pub partitions_total: Option<i64>,
    /// Partitions done.
    pub partitions_done: Option<i64>,
}
