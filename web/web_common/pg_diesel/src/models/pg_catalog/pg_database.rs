//! Submodule providing the `PgDatabase` struct representing a row of the
//! `pg_database` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_database` table.
///
/// The `pg_database` system catalog stores information about available
/// databases. Most of the information shown in this catalog is also available
/// via the `\l` command in psql.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-database.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_database::pg_database)]
pub struct PgDatabase {
    /// OID of the database.
    pub oid: u32,
    /// Name of the database.
    pub datname: String,
    /// OID of the role that owns the database.
    pub datdba: u32,
    /// Character encoding for this database.
    pub encoding: i32,
    /// Locale provider.
    pub datlocprovider: String,
    /// Whether this is a template database.
    pub datistemplate: bool,
    /// Whether connections to this database are allowed.
    pub datallowconn: bool,
    /// Whether login events are logged for this database.
    pub dathasloginevt: bool,
    /// Maximum number of concurrent connections.
    pub datconnlimit: i32,
    /// All transaction IDs before this one have been replaced.
    pub datfrozenxid: u32,
    /// All multixact IDs before this one have been replaced.
    pub datminmxid: u32,
    /// Default tablespace for this database.
    pub dattablespace: u32,
    /// LC_COLLATE setting for this database.
    pub datcollate: String,
    /// LC_CTYPE setting for this database.
    pub datctype: String,
    /// Locale name if using ICU provider.
    pub datlocale: Option<String>,
    /// ICU collation rules if using ICU provider.
    pub daticurules: Option<String>,
    /// Version of the collation.
    pub datcollversion: Option<String>,
    /// Access privileges (ACL) for the database.
    pub datacl: Option<Vec<String>>,
}
