//! Submodule for the `pg_catalog.pg_replication_origin` table schema.

diesel::table! {
    /// `pg_catalog.pg_replication_origin` — system catalog containing replication origins.
    /// Each row represents a replication origin for tracking replication progress.
    pg_catalog.pg_replication_origin (roident) {
        /// OID of the replication origin.
        roident -> Oid,
        /// External name of the replication origin.
        roname -> Text,
    }
}
