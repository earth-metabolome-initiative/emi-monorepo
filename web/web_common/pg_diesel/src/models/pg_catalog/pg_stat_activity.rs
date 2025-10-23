//! Submodule providing the `PgStatActivity` struct representing a row of the
//! `pg_stat_activity` view in `PostgreSQL`.

use std::{net::IpAddr, time::SystemTime};

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_stat_activity` view.
///
/// The `pg_stat_activity` view shows one row per server process with details
/// about the current activity of that process.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/monitoring-stats.html#MONITORING-PG-STAT-ACTIVITY-VIEW).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_stat_activity::pg_stat_activity)]
pub struct PgStatActivity {
    /// Database OID.
    pub datid: Option<u32>,
    /// Database name.
    pub datname: Option<String>,
    /// Process ID.
    pub pid: Option<i32>,
    /// Parallel group leader PID.
    pub leader_pid: Option<i32>,
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
    /// Transaction start time.
    pub xact_start: Option<SystemTime>,
    /// Query start time.
    pub query_start: Option<SystemTime>,
    /// State change time.
    pub state_change: Option<SystemTime>,
    /// Wait event type.
    pub wait_event_type: Option<String>,
    /// Wait event.
    pub wait_event: Option<String>,
    /// Current state.
    pub state: Option<String>,
    /// Backend transaction ID.
    pub backend_xid: Option<u32>,
    /// Backend transaction snapshot.
    pub backend_xmin: Option<u32>,
    /// Query ID.
    pub query_id: Option<i64>,
    /// Query text.
    pub query: Option<String>,
    /// Backend type.
    pub backend_type: Option<String>,
}
