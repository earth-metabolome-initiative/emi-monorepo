//! Submodule providing the `PgReplicationOriginStatus` struct representing a
//! row of the `pg_replication_origin_status` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_replication_origin_status` view.
///
/// The `pg_replication_origin_status` view shows the current state of all
/// replication origins, including how far replication has progressed.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-replication-origin-status.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_replication_origin_status::pg_replication_origin_status)]
pub struct PgReplicationOriginStatus {
    /// Internal OID of the replication origin.
    pub local_id: Option<u32>,
    /// External name of the origin.
    pub external_id: Option<String>,
    /// Remote LSN position.
    pub remote_lsn: Option<u64>,
    /// Local LSN position.
    pub local_lsn: Option<u64>,
}
