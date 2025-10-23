//! Submodule providing the `PgStatBgwriter` struct representing a row of the
//! `pg_stat_bgwriter` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_bgwriter` view.
///
/// The `pg_stat_bgwriter` view shows statistics about the background writer
/// process's activity. It contains only one row with cluster-wide data.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-BGWRITER-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_bgwriter::pg_stat_bgwriter)]
pub struct PgStatBgwriter {
    /// Buffers written.
    pub buffers_clean: Option<i64>,
    /// Maxwritten clean stops.
    pub maxwritten_clean: Option<i64>,
    /// Buffers allocated.
    pub buffers_alloc: Option<i64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
