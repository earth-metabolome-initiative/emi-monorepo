//! Submodule for the `pg_catalog.pg_stat_user_indexes` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_user_indexes` — view showing statistics for user-defined indexes.
    /// Each row represents one user-defined index showing statistics about accesses to that index.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_user_indexes (relid) {
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
        /// Number of index scans initiated on this index.
        idx_scan -> Nullable<BigInt>,
        /// Time of the last scan on this index.
        last_idx_scan -> Nullable<Timestamp>,
        /// Number of index entries returned by scans on this index.
        idx_tup_read -> Nullable<BigInt>,
        /// Number of live table rows fetched by simple index scans using this index.
        idx_tup_fetch -> Nullable<BigInt>,
    }
}
