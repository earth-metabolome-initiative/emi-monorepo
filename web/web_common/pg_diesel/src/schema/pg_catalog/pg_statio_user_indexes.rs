//! Submodule for the `pg_catalog.pg_statio_user_indexes` view schema.

diesel::table! {
    /// `pg_catalog.pg_statio_user_indexes` — view showing I/O statistics for user-defined indexes.
    /// Each row represents one user-defined index showing statistics about I/O operations on that index.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_statio_user_indexes (relid) {
        /// OID of the table for this index.
        relid -> Nullable<Oid>,
        /// OID of this index.
        indexrelid -> Nullable<Oid>,
        /// Name of the schema that this index is in.
        schemaname -> Nullable<Text>,
        /// Name of the table for this index.
        relname -> Nullable<Text>,
        /// Name of this index.
        indexrelname -> Nullable<Text>,
        /// Number of disk blocks read from this index.
        idx_blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this index.
        idx_blks_hit -> Nullable<BigInt>,
    }
}
