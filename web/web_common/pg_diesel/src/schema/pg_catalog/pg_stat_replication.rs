//! Submodule for the `pg_catalog.pg_stat_replication` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_replication` — view showing replication statistics.
    /// Each row represents one WAL sender process showing information about replication to that sender's connected standby server.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_replication (pid) {
        /// Process ID of the WAL sender process.
        pid -> Nullable<Integer>,
        /// OID of the user logged into this WAL sender process.
        usesysid -> Nullable<Oid>,
        /// Name of the user logged into this WAL sender process.
        usename -> Nullable<Text>,
        /// Name of the application connected to this WAL sender.
        application_name -> Nullable<Text>,
        /// IP address of the client connected to this WAL sender.
        client_addr -> Nullable<Inet>,
        /// Hostname of the connected client.
        client_hostname -> Nullable<Text>,
        /// TCP port number that the client is using for communication.
        client_port -> Nullable<Integer>,
        /// Time when this process was started.
        backend_start -> Nullable<Timestamp>,
        /// Hot standby feedback sent by this standby server.
        backend_xmin -> Nullable<Oid>,
        /// Current WAL sender state.
        state -> Nullable<Text>,
        /// Last write-ahead log location sent on this connection.
        sent_lsn -> Nullable<PgLsn>,
        /// Last write-ahead log location written to disk by this standby server.
        write_lsn -> Nullable<PgLsn>,
        /// Last write-ahead log location flushed to disk by this standby server.
        flush_lsn -> Nullable<PgLsn>,
        /// Last write-ahead log location replayed into the database on this standby server.
        replay_lsn -> Nullable<PgLsn>,
        /// Time elapsed between flushing recent WAL locally and receiving notification that this standby has written it.
        write_lag -> Nullable<Interval>,
        /// Time elapsed between flushing recent WAL locally and receiving notification that this standby has flushed it.
        flush_lag -> Nullable<Interval>,
        /// Time elapsed between flushing recent WAL locally and receiving notification that this standby has replayed it.
        replay_lag -> Nullable<Interval>,
        /// Priority of this standby for being chosen as the synchronous standby.
        sync_priority -> Nullable<Integer>,
        /// Synchronous state of this standby server.
        sync_state -> Nullable<Text>,
        /// Send time of last reply message received from standby server.
        reply_time -> Nullable<Timestamp>,
    }
}
