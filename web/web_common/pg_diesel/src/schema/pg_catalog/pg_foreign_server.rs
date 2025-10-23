//! Submodule for the `pg_catalog.pg_foreign_server` table schema.

diesel::table! {
    /// `pg_catalog.pg_foreign_server` — system catalog containing foreign servers.
    /// Each row describes a foreign server, which represents a connection to an
    /// external data source.
    pg_catalog.pg_foreign_server (oid) {
        /// OID of the foreign server.
        oid -> Oid,
        /// Name of the foreign server.
        srvname -> Text,
        /// OID of the role that owns the foreign server.
        srvowner -> Oid,
        /// OID of the foreign data wrapper for this server.
        srvfdw -> Oid,
        /// Optional server type string, as defined by the foreign data wrapper.
        srvtype -> Nullable<Text>,
        /// Optional server version string, as defined by the foreign data wrapper.
        srvversion -> Nullable<Text>,
        /// Access privileges for the foreign server.
        srvacl -> Nullable<Array<Text>>,
        /// Foreign server-specific options, stored as "name=value" strings.
        srvoptions -> Nullable<Array<Text>>,
    }
}
