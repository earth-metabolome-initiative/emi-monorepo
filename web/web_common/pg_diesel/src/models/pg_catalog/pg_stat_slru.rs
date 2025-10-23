//! Submodule providing the `PgStatSlru` struct representing a row of the
//! `pg_stat_slru` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_slru` view.
///
/// The `pg_stat_slru` view shows statistics about SLRU (Simple Least Recently
/// Used) caches, which are used for various internal caching purposes.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-SLRU-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_slru::pg_stat_slru)]
pub struct PgStatSlru {
    /// SLRU name.
    pub name: Option<String>,
    /// Blocks zeroed.
    pub blks_zeroed: Option<i64>,
    /// Buffer hits.
    pub blks_hit: Option<i64>,
    /// Blocks read.
    pub blks_read: Option<i64>,
    /// Blocks written.
    pub blks_written: Option<i64>,
    /// Blocks checked for existence.
    pub blks_exists: Option<i64>,
    /// Flushes.
    pub flushes: Option<i64>,
    /// Truncates.
    pub truncates: Option<i64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
