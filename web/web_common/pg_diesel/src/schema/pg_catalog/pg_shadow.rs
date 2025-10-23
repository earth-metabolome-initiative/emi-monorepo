//! Submodule for the `pg_catalog.pg_shadow` view schema.

diesel::table! {
    /// `pg_catalog.pg_shadow` — view showing information about database users (superuser-only).
    /// This view contains the same information as pg_user, but includes the password field.
    /// Uses `usename` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_shadow (usename) {
        /// User name.
        usename -> Nullable<Text>,
        /// User ID (OID).
        usesysid -> Nullable<Oid>,
        /// Whether the user can create databases.
        usecreatedb -> Nullable<Bool>,
        /// Whether the user is a superuser.
        usesuper -> Nullable<Bool>,
        /// Whether the user can initiate streaming replication.
        userepl -> Nullable<Bool>,
        /// Whether the user can bypass row-level security.
        usebypassrls -> Nullable<Bool>,
        /// Encrypted password (null if none).
        passwd -> Nullable<Text>,
        /// Password expiry time (null if no expiration).
        valuntil -> Nullable<Timestamp>,
        /// Session defaults for run-time configuration variables.
        useconfig -> Nullable<Array<Text>>,
    }
}
