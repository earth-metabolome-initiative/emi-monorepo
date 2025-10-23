//! Submodule for the `pg_catalog.pg_stat_subscription_stats` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_subscription_stats` — view showing subscription error statistics.
    /// Each row represents one subscription showing error statistics for that subscription.
    /// Uses `subid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_subscription_stats (subid) {
        /// OID of the subscription.
        subid -> Nullable<Oid>,
        /// Name of the subscription.
        subname -> Nullable<Text>,
        /// Number of times an apply worker has errored.
        apply_error_count -> Nullable<BigInt>,
        /// Number of times a sync worker has errored.
        sync_error_count -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
