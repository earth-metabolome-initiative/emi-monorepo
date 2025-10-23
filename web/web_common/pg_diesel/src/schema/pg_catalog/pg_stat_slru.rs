//! Submodule for the `pg_catalog.pg_stat_slru` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_slru` — view showing SLRU (Simple Least Recently Used) cache statistics.
    /// Each row represents one SLRU cache showing statistics about that cache's usage.
    /// Uses `name` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_slru (name) {
        /// Name of the SLRU.
        name -> Nullable<Text>,
        /// Number of blocks zeroed during initializations.
        blks_zeroed -> Nullable<BigInt>,
        /// Number of times disk blocks were found already in the SLRU.
        blks_hit -> Nullable<BigInt>,
        /// Number of disk blocks read for this SLRU.
        blks_read -> Nullable<BigInt>,
        /// Number of disk blocks written for this SLRU.
        blks_written -> Nullable<BigInt>,
        /// Number of blocks checked for existence for this SLRU.
        blks_exists -> Nullable<BigInt>,
        /// Number of flushes of dirty data for this SLRU.
        flushes -> Nullable<BigInt>,
        /// Number of truncates for this SLRU.
        truncates -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
