//! Submodule for the `pg_catalog.pg_stat_bgwriter` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_bgwriter` — view showing background writer statistics.
    /// Contains a single row showing cluster-wide background writer statistics.
    /// Uses `buffers_clean` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_bgwriter (buffers_clean) {
        /// Number of buffers written by the background writer.
        buffers_clean -> Nullable<BigInt>,
        /// Number of times the background writer stopped a cleaning scan because it had written too many buffers.
        maxwritten_clean -> Nullable<BigInt>,
        /// Number of buffers allocated.
        buffers_alloc -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
