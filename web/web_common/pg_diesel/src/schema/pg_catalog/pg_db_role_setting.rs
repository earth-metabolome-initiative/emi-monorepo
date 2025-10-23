//! Submodule for the `pg_catalog.pg_db_role_setting` table schema.

diesel::table! {
    /// `pg_catalog.pg_db_role_setting` — system catalog storing per-role and per-database
    /// configuration settings. These settings override the server defaults and are applied
    /// when a specific role connects to a specific database.
    pg_catalog.pg_db_role_setting (setdatabase, setrole) {
        /// OID of the database this setting applies to (0 = all databases).
        setdatabase -> Oid,
        /// OID of the role this setting applies to (0 = all roles).
        setrole -> Oid,
        /// Array of configuration parameter settings (in "name=value" format).
        setconfig -> Nullable<Array<Text>>,
    }
}
