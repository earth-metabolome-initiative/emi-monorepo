//! Submodule providing the `PgStatXactSysTable` struct representing a row of
//! the `pg_stat_xact_sys_tables` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_xact_sys_tables` view.
///
/// The `pg_stat_xact_sys_tables` view shows statistics about accesses to
/// system catalog tables within the current transaction.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-ALL-TABLES-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_xact_sys_tables::pg_stat_xact_sys_tables)]
pub struct PgStatXactSysTable {
    /// Table OID.
    pub relid: Option<u32>,
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub relname: Option<String>,
    /// Sequential scans in transaction.
    pub seq_scan: Option<i64>,
    /// Sequential tuples read in transaction.
    pub seq_tup_read: Option<i64>,
    /// Index scans in transaction.
    pub idx_scan: Option<i64>,
    /// Index tuples fetched in transaction.
    pub idx_tup_fetch: Option<i64>,
    /// Rows inserted in transaction.
    pub n_tup_ins: Option<i64>,
    /// Rows updated in transaction.
    pub n_tup_upd: Option<i64>,
    /// Rows deleted in transaction.
    pub n_tup_del: Option<i64>,
    /// HOT updates in transaction.
    pub n_tup_hot_upd: Option<i64>,
    /// New page updates in transaction.
    pub n_tup_newpage_upd: Option<i64>,
}
