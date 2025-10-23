//! Submodule providing the `PgForeignServer` struct representing a row of the
//! `pg_foreign_server` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_foreign_server` table.
///
/// The `pg_foreign_server` system catalog stores foreign server definitions.
/// A foreign server typically holds connection information that a foreign data
/// wrapper uses to access an external data resource.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-foreign-server.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_foreign_server::pg_foreign_server)]
pub struct PgForeignServer {
    /// OID of the foreign server.
    pub oid: u32,
    /// Name of the foreign server.
    pub srvname: String,
    /// OID of the role that owns the foreign server.
    pub srvowner: u32,
    /// OID of the foreign data wrapper.
    pub srvfdw: u32,
    /// Optional server type string.
    pub srvtype: Option<String>,
    /// Optional server version string.
    pub srvversion: Option<String>,
    /// Access privileges.
    pub srvacl: Option<Vec<String>>,
    /// Foreign server-specific options.
    pub srvoptions: Option<Vec<String>>,
}
