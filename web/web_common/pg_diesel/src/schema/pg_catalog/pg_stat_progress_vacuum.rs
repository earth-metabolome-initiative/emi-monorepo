//! Submodule for the `pg_catalog.pg_stat_progress_vacuum` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_vacuum` — view showing VACUUM command progress.
    /// Each row represents one backend running VACUUM showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_vacuum (pid) {
        /// Process ID of the backend running VACUUM.
        pid -> Nullable<Integer>,
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// OID of the table being vacuumed.
        relid -> Nullable<Oid>,
        /// Current processing phase.
        phase -> Nullable<Text>,
        /// Total number of heap blocks in the table.
        heap_blks_total -> Nullable<BigInt>,
        /// Number of heap blocks scanned.
        heap_blks_scanned -> Nullable<BigInt>,
        /// Number of heap blocks vacuumed.
        heap_blks_vacuumed -> Nullable<BigInt>,
        /// Number of completed index vacuum cycles.
        index_vacuum_count -> Nullable<BigInt>,
        /// Maximum bytes allowed for dead tuples before index vacuum.
        max_dead_tuple_bytes -> Nullable<BigInt>,
        /// Current bytes of dead tuples.
        dead_tuple_bytes -> Nullable<BigInt>,
        /// Number of dead item identifiers.
        num_dead_item_ids -> Nullable<BigInt>,
        /// Total number of indexes on the table.
        indexes_total -> Nullable<BigInt>,
        /// Number of indexes processed.
        indexes_processed -> Nullable<BigInt>,
    }
}
