//! Submodule for the `pg_catalog.pg_shdescription` table schema.

diesel::table! {
    /// `pg_catalog.pg_shdescription` — system catalog containing descriptions of shared objects.
    /// Each row stores a comment for a shared database object like a role or tablespace.
    pg_catalog.pg_shdescription (objoid, classoid) {
        /// OID of the shared object.
        objoid -> Oid,
        /// OID of the system catalog containing the object.
        classoid -> Oid,
        /// Descriptive text for the object.
        description -> Text,
    }
}
