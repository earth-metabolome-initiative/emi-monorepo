//! Submodule for the `pg_catalog.pg_stat_replication_slots` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_replication_slots` — view showing replication slot statistics.
    /// Each row represents one replication slot showing statistics about that slot's usage.
    /// Uses `slot_name` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_replication_slots (slot_name) {
        /// Name of the replication slot.
        slot_name -> Nullable<Text>,
        /// Number of transactions spilled to disk.
        spill_txns -> Nullable<BigInt>,
        /// Number of times transactions were spilled to disk.
        spill_count -> Nullable<BigInt>,
        /// Amount of decoded transaction data spilled to disk (bytes).
        spill_bytes -> Nullable<BigInt>,
        /// Number of in-progress transactions streamed to the decoding output plugin.
        stream_txns -> Nullable<BigInt>,
        /// Number of times in-progress transactions were streamed.
        stream_count -> Nullable<BigInt>,
        /// Amount of in-progress transaction data decoded (bytes).
        stream_bytes -> Nullable<BigInt>,
        /// Number of decoded transactions sent to the decoding output plugin.
        total_txns -> Nullable<BigInt>,
        /// Amount of transaction data decoded (bytes).
        total_bytes -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
