//! Schema for pg_catalog.pg_user_mapping table.

diesel::table! {
    use diesel::sql_types::*;

    /// User mappings
    ///
    /// The catalog `pg_user_mapping` stores the mappings from local users to
    /// users on remote servers. Access to this catalog is restricted from normal users.
    pg_catalog.pg_user_mapping (oid) {
        /// Row identifier
        oid -> Oid,
        /// OID of the local user being mapped, or 0 if the mapping is public
        umuser -> Oid,
        /// OID of the foreign server that contains this mapping
        umserver -> Oid,
        /// User mapping specific options, as "keyword=value" strings
        umoptions -> Nullable<Array<Text>>,
    }
}
