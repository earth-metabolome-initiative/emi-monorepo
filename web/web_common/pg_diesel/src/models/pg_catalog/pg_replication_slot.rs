//! Submodule providing the `PgReplicationSlot` struct representing a row of the
//! `pg_replication_slots` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_replication_slots` view.
///
/// The `pg_replication_slots` view provides information about all replication
/// slots that currently exist on the database cluster, along with their current
/// state.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-replication-slots.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_replication_slots::pg_replication_slots)]
pub struct PgReplicationSlot {
    /// Slot name.
    pub slot_name: Option<String>,
    /// Output plugin name.
    pub plugin: Option<String>,
    /// Slot type.
    pub slot_type: Option<String>,
    /// Database OID.
    pub datoid: Option<u32>,
    /// Database name.
    pub database: Option<String>,
    /// Whether temporary.
    pub temporary: Option<bool>,
    /// Whether active.
    pub active: Option<bool>,
    /// Active session PID.
    pub active_pid: Option<i32>,
    /// Oldest transaction to retain.
    pub xmin: Option<u32>,
    /// Oldest catalog transaction to retain.
    pub catalog_xmin: Option<u32>,
    /// Restart LSN position.
    pub restart_lsn: Option<u64>,
    /// Confirmed flush LSN position.
    pub confirmed_flush_lsn: Option<u64>,
    /// WAL status.
    pub wal_status: Option<String>,
    /// Safe WAL size in bytes.
    pub safe_wal_size: Option<i64>,
    /// Whether two-phase commit enabled.
    pub two_phase: Option<bool>,
    /// Time since inactive.
    pub inactive_since: Option<std::time::SystemTime>,
    /// Whether conflicting.
    pub conflicting: Option<bool>,
    /// Invalidation reason.
    pub invalidation_reason: Option<String>,
    /// Whether failover enabled.
    pub failover: Option<bool>,
    /// Whether synced from primary.
    pub synced: Option<bool>,
}
