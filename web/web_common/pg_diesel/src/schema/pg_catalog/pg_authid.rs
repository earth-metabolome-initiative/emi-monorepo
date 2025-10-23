//! Submodule for the `pg_catalog.pg_authid` table schema.

diesel::table! {
    /// `pg_catalog.pg_authid` — catalog of authorization identifiers (roles).
    /// Contains information about database roles including their privileges and properties.
    pg_catalog.pg_authid (oid) {
        /// OID of the role (primary key).
        oid -> diesel::sql_types::Oid,
        /// Name of the role.
        rolname -> Text,
        /// Whether role has superuser privileges.
        rolsuper -> Bool,
        /// Whether role inherits privileges of roles it is a member of.
        rolinherit -> Bool,
        /// Whether role can create other roles.
        rolcreaterole -> Bool,
        /// Whether role can create databases.
        rolcreatedb -> Bool,
        /// Whether role can log in (has login privilege).
        rolcanlogin -> Bool,
        /// Whether role can initiate streaming replication.
        rolreplication -> Bool,
        /// Whether role can bypass row-level security policies.
        rolbypassrls -> Bool,
        /// Maximum number of concurrent connections for this role (-1 = no limit).
        rolconnlimit -> Integer,
        /// Encrypted password for the role.
        rolpassword -> Nullable<Text>,
        /// Password expiry time.
        rolvaliduntil -> Nullable<Timestamp>,
    }
}
