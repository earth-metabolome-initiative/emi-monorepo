//! Submodule providing the `PgStatProgressVacuum` struct representing a row of
//! the `pg_stat_progress_vacuum` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_progress_vacuum` view.
///
/// The `pg_stat_progress_vacuum` view shows progress information for each
/// backend running VACUUM.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/progress-reporting.html#VACUUM-PROGRESS-REPORTING).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_progress_vacuum::pg_stat_progress_vacuum)]
pub struct PgStatProgressVacuum {
    /// Process ID.
    pub pid: Option<i32>,
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Table OID.
    pub relid: Option<u32>,
    /// Current phase.
    pub phase: Option<String>,
    /// Total heap blocks.
    pub heap_blks_total: Option<i64>,
    /// Heap blocks scanned.
    pub heap_blks_scanned: Option<i64>,
    /// Heap blocks vacuumed.
    pub heap_blks_vacuumed: Option<i64>,
    /// Index vacuum cycles.
    pub index_vacuum_count: Option<i64>,
    /// Max dead tuple bytes.
    pub max_dead_tuple_bytes: Option<i64>,
    /// Current dead tuple bytes.
    pub dead_tuple_bytes: Option<i64>,
    /// Dead item identifiers.
    pub num_dead_item_ids: Option<i64>,
    /// Total indexes.
    pub indexes_total: Option<i64>,
    /// Indexes processed.
    pub indexes_processed: Option<i64>,
}
