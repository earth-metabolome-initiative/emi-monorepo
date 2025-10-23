//! Submodule for the `pg_catalog.pg_seclabel` table schema.

diesel::table! {
    /// `pg_catalog.pg_seclabel` — system catalog containing security labels.
    /// Each row represents a security label assigned to a database object.
    pg_catalog.pg_seclabel (objoid, classoid, objsubid, provider) {
        /// OID of the object this label applies to.
        objoid -> Oid,
        /// OID of the system catalog containing the object.
        classoid -> Oid,
        /// Sub-object ID (e.g., column number for a column).
        objsubid -> Integer,
        /// Name of the security label provider.
        provider -> Text,
        /// Security label text.
        label -> Text,
    }
}
