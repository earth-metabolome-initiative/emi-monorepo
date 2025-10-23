//! Submodule providing the `PgStatioSysIndex` struct representing a row of the
//! `pg_statio_sys_indexes` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_statio_sys_indexes` view.
///
/// The `pg_statio_sys_indexes` view shows I/O statistics about accesses to
/// system catalog indexes.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STATIO-ALL-INDEXES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_statio_sys_indexes::pg_statio_sys_indexes)]
pub struct PgStatioSysIndex {
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
    /// Disk blocks read.
    pub idx_blks_read: Option<i64>,
    /// Buffer hits.
    pub idx_blks_hit: Option<i64>,
}
