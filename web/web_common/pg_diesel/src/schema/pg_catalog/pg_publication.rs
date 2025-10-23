//! Submodule for the `pg_catalog.pg_publication` table schema.

diesel::table! {
    /// `pg_catalog.pg_publication` — system catalog containing logical replication publications.
    /// Each row represents a publication for use with logical replication.
    pg_catalog.pg_publication (oid) {
        /// OID of the publication.
        oid -> Oid,
        /// Name of the publication.
        pubname -> Text,
        /// OID of the owner of the publication.
        pubowner -> Oid,
        /// `true` if the publication automatically includes all tables in the database.
        puballtables -> Bool,
        /// `true` if INSERT operations are published.
        pubinsert -> Bool,
        /// `true` if UPDATE operations are published.
        pubupdate -> Bool,
        /// `true` if DELETE operations are published.
        pubdelete -> Bool,
        /// `true` if TRUNCATE operations are published.
        pubtruncate -> Bool,
        /// `true` if operations on partitions are published via the root partitioned table.
        pubviaroot -> Bool,
    }
}
