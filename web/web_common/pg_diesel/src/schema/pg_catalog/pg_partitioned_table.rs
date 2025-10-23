//! Submodule for the `pg_catalog.pg_partitioned_table` table schema.

diesel::table! {
    /// `pg_catalog.pg_partitioned_table` — system catalog containing partitioning information for partitioned tables.
    /// Each row represents a partitioned table and describes its partitioning strategy.
    pg_catalog.pg_partitioned_table (partrelid) {
        /// OID of the partitioned table (references pg_class).
        partrelid -> Oid,
        /// Partitioning strategy: 'l' (list), 'r' (range), 'h' (hash).
        partstrat -> Text,
        /// Number of columns in the partition key.
        partnatts -> SmallInt,
        /// OID of the default partition (0 if none).
        partdefid -> Oid,
        /// Array of attribute numbers of the partition key columns.
        partattrs -> Array<SmallInt>,
        /// Array of operator class OIDs for the partition key columns.
        partclass -> Array<Oid>,
        /// Array of collation OIDs for the partition key columns.
        partcollation -> Array<Oid>,
        /// Partition key expressions as a node-tree representation.
        partexprs -> Nullable<Text>,
    }
}
