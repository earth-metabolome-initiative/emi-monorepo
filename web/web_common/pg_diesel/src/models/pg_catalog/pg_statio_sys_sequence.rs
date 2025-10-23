//! Submodule providing the `PgStatioSysSequence` struct representing a row of
//! the `pg_statio_sys_sequences` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_statio_sys_sequences` view.
///
/// The `pg_statio_sys_sequences` view shows I/O statistics about accesses to
/// system catalog sequences.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STATIO-ALL-SEQUENCES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_statio_sys_sequences::pg_statio_sys_sequences)]
pub struct PgStatioSysSequence {
    /// Sequence OID.
    pub relid: Option<u32>,
    /// Schema name.
    pub schemaname: Option<String>,
    /// Sequence name.
    pub relname: Option<String>,
    /// Disk blocks read.
    pub blks_read: Option<i64>,
    /// Buffer hits.
    pub blks_hit: Option<i64>,
}
