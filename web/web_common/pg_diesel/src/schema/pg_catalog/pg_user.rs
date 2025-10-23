//! Schema for pg_catalog.pg_user view.

diesel::table! {
    use diesel::sql_types::*;

    /// Database users
    ///
    /// The view `pg_user` provides information about database users.
    /// This is a publicly readable view of `pg_authid` that blanks out the password field.
    pg_catalog.pg_user (usename) {
        /// User name
        usename -> Nullable<Text>,
        /// User ID
        usesysid -> Nullable<Oid>,
        /// User can create databases
        usecreatedb -> Nullable<Bool>,
        /// User is a superuser
        usesuper -> Nullable<Bool>,
        /// User can initiate streaming replication
        userepl -> Nullable<Bool>,
        /// User can bypass row-level security
        usebypassrls -> Nullable<Bool>,
        /// Password (always reads as ********)
        passwd -> Nullable<Text>,
        /// Password expiry time
        valuntil -> Nullable<Timestamp>,
        /// Session defaults for run-time configuration variables
        useconfig -> Nullable<Array<Text>>,
    }
}
