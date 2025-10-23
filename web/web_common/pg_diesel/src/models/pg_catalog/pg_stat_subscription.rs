//! Submodule providing the `PgStatSubscription` struct representing a row of
//! the `pg_stat_subscription` view in `PostgreSQL`.

use std::time::SystemTime;

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_subscription` view.
///
/// The `pg_stat_subscription` view shows statistics about subscription workers.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-SUBSCRIPTION).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_subscription::pg_stat_subscription)]
pub struct PgStatSubscription {
    /// Subscription OID.
    pub subid: Option<u32>,
    /// Subscription name.
    pub subname: Option<String>,
    /// Worker type.
    pub worker_type: Option<String>,
    /// Process ID.
    pub pid: Option<i32>,
    /// Leader process ID.
    pub leader_pid: Option<i32>,
    /// Relation OID.
    pub relid: Option<u32>,
    /// Received LSN.
    pub received_lsn: Option<u64>,
    /// Last message send time.
    pub last_msg_send_time: Option<SystemTime>,
    /// Last message receipt time.
    pub last_msg_receipt_time: Option<SystemTime>,
    /// Latest end LSN.
    pub latest_end_lsn: Option<u64>,
    /// Latest end time.
    pub latest_end_time: Option<SystemTime>,
}
