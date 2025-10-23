//! PostgreSQL authorization identifier membership catalog model.
//!
//! This module provides the `PgAuthMembers` struct for working with the
//! `pg_catalog.pg_auth_members` system catalog table.

/// Represents a row from the `pg_catalog.pg_auth_members` table.
///
/// Contains information about which roles are members of which other roles.
/// This table tracks the membership relationships between PostgreSQL roles
/// and the privileges associated with those memberships.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_auth_members::pg_auth_members)]
pub struct PgAuthMembers {
    /// OID of this membership relationship (primary key).
    pub oid: u32,
    /// OID of the role that has members.
    pub roleid: u32,
    /// OID of the role that is a member.
    pub member: u32,
    /// OID of the role that granted this membership.
    pub grantor: u32,
    /// Whether the member can grant this role to others.
    pub admin_option: bool,
    /// Whether the member inherits privileges of this role.
    pub inherit_option: bool,
    /// Whether the member can SET ROLE to this role.
    pub set_option: bool,
}
