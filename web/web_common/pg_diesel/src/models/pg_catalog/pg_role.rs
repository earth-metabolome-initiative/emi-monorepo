//! Submodule providing the `PgRole` struct representing a row of the
//! `pg_roles` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_roles` view.
///
/// The `pg_roles` view provides information about database roles. It is a
/// publicly readable view of `pg_authid` that blanks out the password field.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-roles.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_roles::pg_roles)]
pub struct PgRole {
    /// Role name.
    pub rolname: Option<String>,
    /// Whether superuser.
    pub rolsuper: Option<bool>,
    /// Whether inherits privileges.
    pub rolinherit: Option<bool>,
    /// Whether can create roles.
    pub rolcreaterole: Option<bool>,
    /// Whether can create databases.
    pub rolcreatedb: Option<bool>,
    /// Whether can login.
    pub rolcanlogin: Option<bool>,
    /// Whether can replicate.
    pub rolreplication: Option<bool>,
    /// Connection limit.
    pub rolconnlimit: Option<i32>,
    /// Hashed password.
    pub rolpassword: Option<String>,
    /// Password expiry time.
    pub rolvaliduntil: Option<std::time::SystemTime>,
    /// Whether can bypass RLS.
    pub rolbypassrls: Option<bool>,
    /// Role configuration.
    pub rolconfig: Option<Vec<String>>,
    /// OID of the role.
    pub oid: Option<u32>,
}
