//! PostgreSQL authorization identifiers (roles) catalog model.
//!
//! This module provides the `PgAuthid` struct for working with the
//! `pg_catalog.pg_authid` system catalog table.

use std::time::SystemTime;

/// Represents a row from the `pg_catalog.pg_authid` table.
///
/// Contains information about database roles including their privileges and
/// properties. This table stores all authentication identifiers (roles) in the
/// PostgreSQL cluster including users, groups, and other role types.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_authid::pg_authid)]
pub struct PgAuthid {
    /// OID of the role (primary key).
    pub oid: u32,
    /// Name of the role.
    pub rolname: String,
    /// Whether role has superuser privileges.
    pub rolsuper: bool,
    /// Whether role inherits privileges of roles it is a member of.
    pub rolinherit: bool,
    /// Whether role can create other roles.
    pub rolcreaterole: bool,
    /// Whether role can create databases.
    pub rolcreatedb: bool,
    /// Whether role can log in (has login privilege).
    pub rolcanlogin: bool,
    /// Whether role can initiate streaming replication.
    pub rolreplication: bool,
    /// Whether role can bypass row-level security policies.
    pub rolbypassrls: bool,
    /// Maximum number of concurrent connections for this role (-1 = no limit).
    pub rolconnlimit: i32,
    /// Encrypted password for the role.
    pub rolpassword: Option<String>,
    /// Password expiry time.
    pub rolvaliduntil: Option<SystemTime>,
}
