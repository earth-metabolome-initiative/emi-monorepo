//! Submodule for the `pg_catalog.pg_am` table schema.

diesel::table! {
    /// `pg_catalog.pg_am` — catalog of index access methods.
    /// Contains information about available index access methods (btree, hash, gin, gist, etc.).
    pg_catalog.pg_am (oid) {
        /// OID of the access method (primary key).
        oid -> diesel::sql_types::Oid,
        /// Name of the access method.
        amname -> Text,
        /// OID of handler function for this access method.
        amhandler -> diesel::sql_types::Oid,
        /// Type of access method (i=index, t=table).
        amtype -> Text,
    }
}
