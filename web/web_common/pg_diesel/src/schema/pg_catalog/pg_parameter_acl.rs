//! Submodule for the `pg_catalog.pg_parameter_acl` table schema.

diesel::table! {
    /// `pg_catalog.pg_parameter_acl` — system catalog containing access control lists for configuration parameters.
    /// Each row stores ACL information for a configuration parameter.
    pg_catalog.pg_parameter_acl (oid) {
        /// OID of the parameter ACL entry.
        oid -> Oid,
        /// Name of the configuration parameter.
        parname -> Text,
        /// Access privileges for the parameter.
        paracl -> Nullable<Array<Text>>,
    }
}
