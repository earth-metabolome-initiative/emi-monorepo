//! Submodule for the `pg_catalog.pg_publication_namespace` table schema.

diesel::table! {
    /// `pg_catalog.pg_publication_namespace` — system catalog mapping publications to schemas.
    /// Each row represents a schema that is part of a publication.
    pg_catalog.pg_publication_namespace (oid) {
        /// OID of this mapping entry.
        oid -> Oid,
        /// OID of the publication (references pg_publication).
        pnpubid -> Oid,
        /// OID of the schema (references pg_namespace).
        pnnspid -> Oid,
    }
}
