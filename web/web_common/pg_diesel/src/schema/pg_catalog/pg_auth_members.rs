//! Submodule for the `pg_catalog.pg_auth_members` table schema.

diesel::table! {
    /// `pg_catalog.pg_auth_members` — catalog of authorization identifier membership relationships.
    /// Contains information about which roles are members of which other roles.
    pg_catalog.pg_auth_members (oid) {
        /// OID of this membership relationship (primary key).
        oid -> diesel::sql_types::Oid,
        /// OID of the role that has members.
        roleid -> diesel::sql_types::Oid,
        /// OID of the role that is a member.
        member -> diesel::sql_types::Oid,
        /// OID of the role that granted this membership.
        grantor -> diesel::sql_types::Oid,
        /// Whether the member can grant this role to others.
        admin_option -> Bool,
        /// Whether the member inherits privileges of this role.
        inherit_option -> Bool,
        /// Whether the member can SET ROLE to this role.
        set_option -> Bool,
    }
}
