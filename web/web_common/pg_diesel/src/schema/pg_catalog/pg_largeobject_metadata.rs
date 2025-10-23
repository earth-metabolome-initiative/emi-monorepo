//! Submodule for the `pg_catalog.pg_largeobject_metadata` table schema.

diesel::table! {
    /// `pg_catalog.pg_largeobject_metadata` — system catalog holding metadata for large objects.
    /// Each large object has one entry here containing ownership and access control information.
    pg_catalog.pg_largeobject_metadata (oid) {
        /// OID of the large object.
        oid -> Oid,
        /// OID of the owner of the large object.
        lomowner -> Oid,
        /// Access privileges for the large object.
        lomacl -> Nullable<Array<Text>>,
    }
}
