//! Submodule providing the `PgStatUserTable` struct representing a row of the
//! `pg_stat_user_tables` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_user_tables` view.
///
/// The `pg_stat_user_tables` view shows statistics about accesses to
/// user-defined tables.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-ALL-TABLES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_user_tables::pg_stat_user_tables)]
pub struct PgStatUserTable {
    /// Table OID.
    pub relid: Option<u32>,
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub relname: Option<String>,
    /// Sequential scans count.
    pub seq_scan: Option<i64>,
    /// Last sequential scan time.
    pub last_seq_scan: Option<SystemTime>,
    /// Sequential tuples read.
    pub seq_tup_read: Option<i64>,
    /// Index scans count.
    pub idx_scan: Option<i64>,
    /// Last index scan time.
    pub last_idx_scan: Option<SystemTime>,
    /// Index tuples fetched.
    pub idx_tup_fetch: Option<i64>,
    /// Rows inserted.
    pub n_tup_ins: Option<i64>,
    /// Rows updated.
    pub n_tup_upd: Option<i64>,
    /// Rows deleted.
    pub n_tup_del: Option<i64>,
    /// HOT updates.
    pub n_tup_hot_upd: Option<i64>,
    /// New page updates.
    pub n_tup_newpage_upd: Option<i64>,
    /// Live tuples estimate.
    pub n_live_tup: Option<i64>,
    /// Dead tuples estimate.
    pub n_dead_tup: Option<i64>,
    /// Modified since analyze.
    pub n_mod_since_analyze: Option<i64>,
    /// Inserted since vacuum.
    pub n_ins_since_vacuum: Option<i64>,
    /// Last vacuum time.
    pub last_vacuum: Option<SystemTime>,
    /// Last autovacuum time.
    pub last_autovacuum: Option<SystemTime>,
    /// Last analyze time.
    pub last_analyze: Option<SystemTime>,
    /// Last autoanalyze time.
    pub last_autoanalyze: Option<SystemTime>,
    /// Vacuum count.
    pub vacuum_count: Option<i64>,
    /// Autovacuum count.
    pub autovacuum_count: Option<i64>,
    /// Analyze count.
    pub analyze_count: Option<i64>,
    /// Autoanalyze count.
    pub autoanalyze_count: Option<i64>,
}
