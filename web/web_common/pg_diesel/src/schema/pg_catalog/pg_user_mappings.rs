//! Schema for pg_catalog.pg_user_mappings view.

diesel::table! {
    use diesel::sql_types::*;

    /// User mappings view
    ///
    /// The view `pg_user_mappings` provides access to information about user mappings.
    /// This is essentially a publicly readable view of `pg_user_mapping` that omits
    /// the options field if the user has no rights to the associated server.
    pg_catalog.pg_user_mappings (umid) {
        /// OID of the user mapping
        umid -> Nullable<Oid>,
        /// OID of the foreign server
        srvid -> Nullable<Oid>,
        /// Name of the foreign server
        srvname -> Nullable<Text>,
        /// OID of the local user being mapped, or 0 if the mapping is public
        umuser -> Nullable<Oid>,
        /// Name of the local user
        usename -> Nullable<Text>,
        /// User mapping specific options
        umoptions -> Nullable<Array<Text>>,
    }
}
