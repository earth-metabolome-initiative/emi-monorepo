//! Submodule for the `pg_catalog.pg_stat_progress_create_index` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_create_index` — view showing index creation progress.
    /// Each row represents one backend creating an index showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_create_index (pid) {
        /// Process ID of the backend creating the index.
        pid -> Nullable<Integer>,
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// OID of the table on which the index is being created.
        relid -> Nullable<Oid>,
        /// OID of the index being created or reindexed.
        index_relid -> Nullable<Oid>,
        /// Command that is running (CREATE INDEX, CREATE INDEX CONCURRENTLY, REINDEX, or REINDEX CONCURRENTLY).
        command -> Nullable<Text>,
        /// Current processing phase.
        phase -> Nullable<Text>,
        /// Total number of lockers to wait for.
        lockers_total -> Nullable<BigInt>,
        /// Number of lockers already waited for.
        lockers_done -> Nullable<BigInt>,
        /// Process ID of the current locker.
        current_locker_pid -> Nullable<BigInt>,
        /// Total number of blocks to be processed.
        blocks_total -> Nullable<BigInt>,
        /// Number of blocks processed.
        blocks_done -> Nullable<BigInt>,
        /// Total number of tuples to be processed.
        tuples_total -> Nullable<BigInt>,
        /// Number of tuples processed.
        tuples_done -> Nullable<BigInt>,
        /// Total number of partitions on which the index is to be created or attached.
        partitions_total -> Nullable<BigInt>,
        /// Number of partitions processed.
        partitions_done -> Nullable<BigInt>,
    }
}
