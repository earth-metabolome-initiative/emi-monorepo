//! Submodule for the `pg_catalog.pg_stat_wal` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_wal` — view showing cluster-wide WAL (Write-Ahead Logging) statistics.
    /// This is a single-row view showing statistics about WAL activity.
    /// Uses `wal_records` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_wal (wal_records) {
        /// Total number of WAL records generated.
        wal_records -> Nullable<BigInt>,
        /// Total number of WAL full page images generated.
        wal_fpi -> Nullable<BigInt>,
        /// Total amount of WAL bytes generated.
        wal_bytes -> Nullable<Double>,
        /// Number of times WAL data was written to disk because WAL buffers became full.
        wal_buffers_full -> Nullable<BigInt>,
        /// Number of times WAL buffers were written out to disk.
        wal_write -> Nullable<BigInt>,
        /// Number of times WAL files were synced to disk.
        wal_sync -> Nullable<BigInt>,
        /// Total time spent writing WAL buffers to disk, in milliseconds.
        wal_write_time -> Nullable<Double>,
        /// Total time spent syncing WAL files to disk, in milliseconds.
        wal_sync_time -> Nullable<Double>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
