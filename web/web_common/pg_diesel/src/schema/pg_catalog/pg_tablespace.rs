//! Submodule for the `pg_catalog.pg_tablespace` table schema.

diesel::table! {
    /// `pg_catalog.pg_tablespace` — table storing tablespaces.
    /// Tablespaces allow database administrators to define locations in the file system
    /// where the files representing database objects can be stored.
    /// Uses `oid` as the primary key.
    pg_catalog.pg_tablespace (oid) {
        /// Row identifier.
        oid -> Oid,
        /// Tablespace name.
        spcname -> Text,
        /// Owner of the tablespace.
        spcowner -> Oid,
        /// Access privileges.
        spcacl -> Nullable<Array<Text>>,
        /// Tablespace-level options.
        spcoptions -> Nullable<Array<Text>>,
    }
}
