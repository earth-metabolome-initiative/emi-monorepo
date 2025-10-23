//! Submodule for the `pg_catalog.pg_stat_checkpointer` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_checkpointer` — view showing checkpointer statistics.
    /// Contains a single row showing cluster-wide checkpointer process statistics.
    /// Uses `num_timed` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_checkpointer (num_timed) {
        /// Number of scheduled checkpoints that have been performed.
        num_timed -> Nullable<BigInt>,
        /// Number of requested checkpoints that have been performed.
        num_requested -> Nullable<BigInt>,
        /// Number of scheduled restartpoints performed (standby servers only).
        restartpoints_timed -> Nullable<BigInt>,
        /// Number of requested restartpoints performed (standby servers only).
        restartpoints_req -> Nullable<BigInt>,
        /// Number of restartpoints completed (standby servers only).
        restartpoints_done -> Nullable<BigInt>,
        /// Total amount of time spent writing files to disk by the checkpointer, in milliseconds.
        write_time -> Nullable<Double>,
        /// Total amount of time spent synchronizing files to disk by the checkpointer, in milliseconds.
        sync_time -> Nullable<Double>,
        /// Number of buffers written during checkpoints and restartpoints.
        buffers_written -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
