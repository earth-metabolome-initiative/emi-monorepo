//! Submodule for the `pg_catalog.pg_stat_wal_receiver` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_wal_receiver` — view showing statistics about the WAL receiver process.
    /// This view contains at most one row showing information about the WAL receiver process on a standby server.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_wal_receiver (pid) {
        /// Process ID of the WAL receiver process.
        pid -> Nullable<Integer>,
        /// Activity status of the WAL receiver process.
        status -> Nullable<Text>,
        /// First write-ahead log location used when WAL receiver is started.
        receive_start_lsn -> Nullable<PgLsn>,
        /// First timeline number used when WAL receiver is started.
        receive_start_tli -> Nullable<Integer>,
        /// Last write-ahead log location already written to disk by this WAL receiver.
        written_lsn -> Nullable<PgLsn>,
        /// Last write-ahead log location already flushed to disk by this WAL receiver.
        flushed_lsn -> Nullable<PgLsn>,
        /// Timeline number of the last write-ahead log location received and flushed to disk by this WAL receiver.
        received_tli -> Nullable<Integer>,
        /// Send time of last message received from origin WAL sender.
        last_msg_send_time -> Nullable<Timestamp>,
        /// Receipt time of last message received from origin WAL sender.
        last_msg_receipt_time -> Nullable<Timestamp>,
        /// Last write-ahead log location reported to origin WAL sender.
        latest_end_lsn -> Nullable<PgLsn>,
        /// Time of last write-ahead log location reported to origin WAL sender.
        latest_end_time -> Nullable<Timestamp>,
        /// Replication slot name used by this WAL receiver.
        slot_name -> Nullable<Text>,
        /// Host of the PostgreSQL instance this WAL receiver is connected to.
        sender_host -> Nullable<Text>,
        /// Port number of the PostgreSQL instance this WAL receiver is connected to.
        sender_port -> Nullable<Integer>,
        /// Connection string used by this WAL receiver.
        conninfo -> Nullable<Text>,
    }
}
