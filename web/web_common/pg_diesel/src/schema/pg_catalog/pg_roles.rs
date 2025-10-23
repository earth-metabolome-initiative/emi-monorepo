//! Submodule for the `pg_catalog.pg_roles` view schema.

diesel::table! {
    /// `pg_catalog.pg_roles` — view showing information about database roles.
    /// Each row represents a role (user or group) with their attributes and privileges.
    /// Uses `rolname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_roles (rolname) {
        /// Name of the role.
        rolname -> Nullable<Text>,
        /// `true` if the role has superuser privileges.
        rolsuper -> Nullable<Bool>,
        /// `true` if the role inherits privileges of roles it is a member of.
        rolinherit -> Nullable<Bool>,
        /// `true` if the role can create new roles.
        rolcreaterole -> Nullable<Bool>,
        /// `true` if the role can create databases.
        rolcreatedb -> Nullable<Bool>,
        /// `true` if the role can log in (i.e., is a user not a group).
        rolcanlogin -> Nullable<Bool>,
        /// `true` if the role can initiate streaming replication.
        rolreplication -> Nullable<Bool>,
        /// Maximum number of concurrent connections (-1 means no limit).
        rolconnlimit -> Nullable<Integer>,
        /// Hashed password (or null).
        rolpassword -> Nullable<Text>,
        /// Password expiry time (null if no expiration).
        rolvaliduntil -> Nullable<Timestamp>,
        /// `true` if the role can bypass row-level security policies.
        rolbypassrls -> Nullable<Bool>,
        /// Role-specific defaults for run-time configuration variables.
        rolconfig -> Nullable<Array<Text>>,
        /// OID of the role.
        oid -> Nullable<Oid>,
    }
}
