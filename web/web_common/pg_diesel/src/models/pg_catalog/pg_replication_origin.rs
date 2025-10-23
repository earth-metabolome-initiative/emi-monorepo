//! Submodule providing the `PgReplicationOrigin` struct representing a row of
//! the `pg_replication_origin` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_replication_origin` table.
///
/// The `pg_replication_origin` catalog contains all created replication
/// origins. Replication origins are used to track replication progress from
/// external systems.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-replication-origin.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_replication_origin::pg_replication_origin)]
pub struct PgReplicationOrigin {
    /// OID of the replication origin.
    pub roident: u32,
    /// Name of the replication origin.
    pub roname: String,
}
