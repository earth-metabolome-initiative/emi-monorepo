//! Submodule providing the `PgStatDatabaseConflict` struct representing a row
//! of the `pg_stat_database_conflicts` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_database_conflicts` view.
///
/// The `pg_stat_database_conflicts` view shows statistics about conflicts
/// that occurred in this database on standby servers.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-DATABASE-CONFLICTS-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_database_conflicts::pg_stat_database_conflicts)]
pub struct PgStatDatabaseConflict {
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Tablespace conflicts.
    pub confl_tablespace: Option<i64>,
    /// Lock conflicts.
    pub confl_lock: Option<i64>,
    /// Snapshot conflicts.
    pub confl_snapshot: Option<i64>,
    /// Buffer pin conflicts.
    pub confl_bufferpin: Option<i64>,
    /// Deadlock conflicts.
    pub confl_deadlock: Option<i64>,
    /// Active logical slot conflicts.
    pub confl_active_logicalslot: Option<i64>,
}
