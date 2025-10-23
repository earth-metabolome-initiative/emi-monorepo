//! Submodule for the `pg_catalog.pg_publication_rel` table schema.

diesel::table! {
    /// `pg_catalog.pg_publication_rel` — system catalog mapping publications to tables.
    /// Each row represents a table that is part of a publication.
    pg_catalog.pg_publication_rel (oid) {
        /// OID of this mapping entry.
        oid -> Oid,
        /// OID of the publication (references pg_publication).
        prpubid -> Oid,
        /// OID of the relation/table (references pg_class).
        prrelid -> Oid,
        /// Array of attribute numbers for column-level replication; `NULL` if all columns.
        prattrs -> Nullable<Array<SmallInt>>,
        /// Optional WHERE clause expression tree for row filtering.
        prqual -> Nullable<Text>,
    }
}
