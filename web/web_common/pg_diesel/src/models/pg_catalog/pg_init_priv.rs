//! Submodule providing the `PgInitPriv` struct representing a row of the
//! `pg_init_privs` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_init_privs` table.
///
/// The `pg_init_privs` system catalog records information about the initial
/// privileges of objects. It is primarily of interest to pg_dump and to
/// extensions that wish to implement proper privilege handling.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-init-privs.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_init_privs::pg_init_privs)]
pub struct PgInitPriv {
    /// OID of the specific object.
    pub objoid: u32,
    /// OID of the system catalog containing the object.
    pub classoid: u32,
    /// Object sub-ID.
    pub objsubid: i32,
    /// Type of privilege.
    pub privtype: String,
    /// Access privileges as originally assigned.
    pub initprivs: Vec<String>,
}
