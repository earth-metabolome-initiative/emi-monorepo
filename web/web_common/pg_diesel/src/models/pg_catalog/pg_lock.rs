//! Submodule providing the `PgLock` struct representing a row of the
//! `pg_locks` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_locks` view.
///
/// The `pg_locks` view provides information about the locks held by active
/// processes within the database server. Each row represents an active lockable
/// object, a requested lock mode, and relevant process information.
///
/// This is a view, not a table, so it has no persistent storage and no primary
/// key.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-locks.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_locks::pg_locks)]
pub struct PgLock {
    /// Type of lockable object.
    pub locktype: Option<String>,
    /// OID of the database.
    pub database: Option<u32>,
    /// OID of the relation.
    pub relation: Option<u32>,
    /// Page number within the relation.
    pub page: Option<i32>,
    /// Tuple number within the page.
    pub tuple: Option<i16>,
    /// Virtual transaction ID.
    pub virtualxid: Option<String>,
    /// Transaction ID.
    pub transactionid: Option<u32>,
    /// OID of the system catalog containing the object.
    pub classid: Option<u32>,
    /// OID of the object.
    pub objid: Option<u32>,
    /// Sub-object ID (e.g., column number).
    pub objsubid: Option<i16>,
    /// Virtual transaction ID of the lock holder/awaiter.
    pub virtualtransaction: Option<String>,
    /// Process ID of the lock holder/awaiter.
    pub pid: Option<i32>,
    /// Name of the lock mode.
    pub mode: Option<String>,
    /// Whether lock is granted.
    pub granted: Option<bool>,
    /// Whether lock was taken via fast path.
    pub fastpath: Option<bool>,
    /// Time when waiting for the lock started.
    pub waitstart: Option<std::time::SystemTime>,
}
