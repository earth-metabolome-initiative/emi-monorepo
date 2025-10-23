//! Submodule providing the `PgStatProgressBasebackup` struct representing a row
//! of the `pg_stat_progress_basebackup` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_progress_basebackup` view.
///
/// The `pg_stat_progress_basebackup` view shows progress information for each
/// WAL sender process streaming a base backup.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/progress-reporting.html#BASEBACKUP-PROGRESS-REPORTING).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_progress_basebackup::pg_stat_progress_basebackup)]
pub struct PgStatProgressBasebackup {
    /// Process ID.
    pub pid: Option<i32>,
    /// Current phase.
    pub phase: Option<String>,
    /// Total backup size (bytes).
    pub backup_total: Option<i64>,
    /// Backup streamed (bytes).
    pub backup_streamed: Option<i64>,
    /// Total tablespaces.
    pub tablespaces_total: Option<i64>,
    /// Tablespaces streamed.
    pub tablespaces_streamed: Option<i64>,
}
