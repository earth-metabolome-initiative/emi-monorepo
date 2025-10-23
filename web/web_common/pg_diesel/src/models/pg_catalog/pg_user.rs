//! Model for pg_catalog.pg_user view.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_user::pg_user)]
#[diesel(primary_key(usename))]
/// Represents a row from the `pg_user` view.
pub struct PgUser {
    /// User name
    pub usename: Option<String>,
    /// User ID
    pub usesysid: Option<u32>,
    /// User can create databases
    pub usecreatedb: Option<bool>,
    /// User is a superuser
    pub usesuper: Option<bool>,
    /// User can initiate streaming replication
    pub userepl: Option<bool>,
    /// User can bypass row-level security
    pub usebypassrls: Option<bool>,
    /// Password (always reads as ********)
    pub passwd: Option<String>,
    /// Password expiry time
    pub valuntil: Option<std::time::SystemTime>,
    /// Session defaults for run-time configuration variables
    pub useconfig: Option<Vec<String>>,
}
