//! Submodule for the `pg_catalog.pg_stat_subscription` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_subscription` — view showing subscription statistics.
    /// Each row represents one subscription worker showing activity information for that worker.
    /// Uses `subid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_subscription (subid) {
        /// OID of the subscription.
        subid -> Nullable<Oid>,
        /// Name of the subscription.
        subname -> Nullable<Text>,
        /// Type of the subscription worker (apply, parallel apply, table sync).
        worker_type -> Nullable<Text>,
        /// Process ID of the subscription worker process.
        pid -> Nullable<Integer>,
        /// Process ID of the leader apply worker, if this is a parallel apply worker.
        leader_pid -> Nullable<Integer>,
        /// OID of the relation that the worker is synchronizing.
        relid -> Nullable<Oid>,
        /// Last write-ahead log location received.
        received_lsn -> Nullable<PgLsn>,
        /// Send time of last message received from origin WAL sender.
        last_msg_send_time -> Nullable<Timestamp>,
        /// Receipt time of last message received from origin WAL sender.
        last_msg_receipt_time -> Nullable<Timestamp>,
        /// Last write-ahead log location reported to origin WAL sender.
        latest_end_lsn -> Nullable<PgLsn>,
        /// Time of last write-ahead log location reported to origin WAL sender.
        latest_end_time -> Nullable<Timestamp>,
    }
}
