//! Submodule providing the `PgStatReplicationSlot` struct representing a row of
//! the `pg_stat_replication_slots` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_replication_slots` view.
///
/// The `pg_stat_replication_slots` view shows statistics about the usage
/// of each replication slot.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-REPLICATION-SLOTS-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_replication_slots::pg_stat_replication_slots)]
pub struct PgStatReplicationSlot {
    /// Slot name.
    pub slot_name: Option<String>,
    /// Spilled transactions.
    pub spill_txns: Option<i64>,
    /// Spill count.
    pub spill_count: Option<i64>,
    /// Spilled bytes.
    pub spill_bytes: Option<i64>,
    /// Streamed transactions.
    pub stream_txns: Option<i64>,
    /// Stream count.
    pub stream_count: Option<i64>,
    /// Streamed bytes.
    pub stream_bytes: Option<i64>,
    /// Total transactions.
    pub total_txns: Option<i64>,
    /// Total bytes.
    pub total_bytes: Option<i64>,
    /// Statistics reset time.
    pub stats_reset: Option<SystemTime>,
}
