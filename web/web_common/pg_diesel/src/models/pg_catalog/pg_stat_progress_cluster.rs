//! Submodule providing the `PgStatProgressCluster` struct representing a row of
//! the `pg_stat_progress_cluster` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_progress_cluster` view.
///
/// The `pg_stat_progress_cluster` view shows progress information for each
/// backend running CLUSTER or VACUUM FULL.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/progress-reporting.html#CLUSTER-PROGRESS-REPORTING).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_progress_cluster::pg_stat_progress_cluster)]
pub struct PgStatProgressCluster {
    /// Process ID.
    pub pid: Option<i32>,
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Table OID.
    pub relid: Option<u32>,
    /// Command (CLUSTER/VACUUM FULL).
    pub command: Option<String>,
    /// Current phase.
    pub phase: Option<String>,
    /// Cluster index OID.
    pub cluster_index_relid: Option<u32>,
    /// Heap tuples scanned.
    pub heap_tuples_scanned: Option<i64>,
    /// Heap tuples written.
    pub heap_tuples_written: Option<i64>,
    /// Heap blocks total.
    pub heap_blks_total: Option<i64>,
    /// Heap blocks scanned.
    pub heap_blks_scanned: Option<i64>,
    /// Index rebuilds.
    pub index_rebuild_count: Option<i64>,
}
