//! Submodule for the `pg_catalog.pg_stat_recovery_prefetch` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_recovery_prefetch` — view showing recovery prefetch statistics.
    /// Contains a single row showing statistics about blocks prefetched during recovery.
    /// Uses `stats_reset` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_recovery_prefetch (stats_reset) {
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
        /// Number of blocks prefetched.
        prefetch -> Nullable<BigInt>,
        /// Number of prefetched blocks that were already in the buffer pool.
        hit -> Nullable<BigInt>,
        /// Number of blocks not prefetched because they were not yet initialized.
        skip_init -> Nullable<BigInt>,
        /// Number of blocks not prefetched because they didn't exist yet.
        skip_new -> Nullable<BigInt>,
        /// Number of blocks not prefetched because a full page image was included in the WAL.
        skip_fpw -> Nullable<BigInt>,
        /// Number of blocks not prefetched because they were already recently accessed.
        skip_rep -> Nullable<BigInt>,
        /// Distance in bytes between current replay position and prefetch position.
        wal_distance -> Nullable<Integer>,
        /// Distance in blocks between current replay position and prefetch position.
        block_distance -> Nullable<Integer>,
        /// Current prefetch depth (number of I/Os in flight).
        io_depth -> Nullable<Integer>,
    }
}
