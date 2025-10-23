//! Submodule providing the `PgStatReplication` struct representing a row of the
//! `pg_stat_replication` view in `PostgreSQL`.

use std::{net::IpAddr, time::SystemTime};

use diesel::{Queryable, QueryableByName, Selectable, pg::data_types::PgInterval};

/// Represents a row from the `pg_stat_replication` view.
///
/// The `pg_stat_replication` view shows statistics about replication
/// to connected standby servers.
///
/// Note: This struct does not derive serde traits because `PgInterval` does not
/// implement them.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-REPLICATION-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone)]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_replication::pg_stat_replication)]
pub struct PgStatReplication {
    /// Process ID.
    pub pid: Option<i32>,
    /// User OID.
    pub usesysid: Option<u32>,
    /// User name.
    pub usename: Option<String>,
    /// Application name.
    pub application_name: Option<String>,
    /// Client IP address.
    pub client_addr: Option<IpAddr>,
    /// Client hostname.
    pub client_hostname: Option<String>,
    /// Client port.
    pub client_port: Option<i32>,
    /// Backend start time.
    pub backend_start: Option<SystemTime>,
    /// Backend transaction snapshot.
    pub backend_xmin: Option<u32>,
    /// Current state.
    pub state: Option<String>,
    /// Sent LSN.
    pub sent_lsn: Option<u64>,
    /// Write LSN.
    pub write_lsn: Option<u64>,
    /// Flush LSN.
    pub flush_lsn: Option<u64>,
    /// Replay LSN.
    pub replay_lsn: Option<u64>,
    /// Write lag.
    pub write_lag: Option<PgInterval>,
    /// Flush lag.
    pub flush_lag: Option<PgInterval>,
    /// Replay lag.
    pub replay_lag: Option<PgInterval>,
    /// Sync priority.
    pub sync_priority: Option<i32>,
    /// Sync state.
    pub sync_state: Option<String>,
    /// Last reply time.
    pub reply_time: Option<SystemTime>,
}
