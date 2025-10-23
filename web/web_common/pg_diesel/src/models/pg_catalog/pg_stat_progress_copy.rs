//! Submodule providing the `PgStatProgressCopy` struct representing a row of
//! the `pg_stat_progress_copy` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_progress_copy` view.
///
/// The `pg_stat_progress_copy` view shows progress information for each
/// backend running COPY.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/progress-reporting.html#COPY-PROGRESS-REPORTING).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_progress_copy::pg_stat_progress_copy)]
pub struct PgStatProgressCopy {
    /// Process ID.
    pub pid: Option<i32>,
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Table OID.
    pub relid: Option<u32>,
    /// Command (COPY FROM/TO).
    pub command: Option<String>,
    /// Entity type (TABLE/PROGRAM).
    pub r#type: Option<String>,
    /// Bytes processed.
    pub bytes_processed: Option<i64>,
    /// Total bytes.
    pub bytes_total: Option<i64>,
    /// Tuples processed.
    pub tuples_processed: Option<i64>,
    /// Tuples excluded.
    pub tuples_excluded: Option<i64>,
    /// Tuples skipped.
    pub tuples_skipped: Option<i64>,
}
