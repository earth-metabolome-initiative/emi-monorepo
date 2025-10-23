//! Submodule for the `pg_catalog.pg_stat_progress_basebackup` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_basebackup` — view showing base backup progress.
    /// Each row represents one WAL sender process streaming a base backup showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_basebackup (pid) {
        /// Process ID of the WAL sender process.
        pid -> Nullable<Integer>,
        /// Current processing phase.
        phase -> Nullable<Text>,
        /// Total amount of data to be streamed in the backup (bytes).
        backup_total -> Nullable<BigInt>,
        /// Amount of data streamed so far (bytes).
        backup_streamed -> Nullable<BigInt>,
        /// Total number of tablespaces to be streamed.
        tablespaces_total -> Nullable<BigInt>,
        /// Number of tablespaces streamed so far.
        tablespaces_streamed -> Nullable<BigInt>,
    }
}
