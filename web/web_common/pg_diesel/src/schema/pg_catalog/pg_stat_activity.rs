//! Submodule for the `pg_catalog.pg_stat_activity` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_activity` — view showing current backend activity.
    /// Each row represents one server process showing current activity of that process.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_activity (pid) {
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// Process ID of this backend.
        pid -> Nullable<Integer>,
        /// Process ID of the parallel group leader, if this is a parallel worker.
        leader_pid -> Nullable<Integer>,
        /// OID of the user logged into this backend.
        usesysid -> Nullable<Oid>,
        /// Name of the user logged into this backend.
        usename -> Nullable<Text>,
        /// Name of the application that is connected to this backend.
        application_name -> Nullable<Text>,
        /// IP address of the client connected to this backend.
        client_addr -> Nullable<Inet>,
        /// Hostname of the client connected to this backend.
        client_hostname -> Nullable<Text>,
        /// TCP port number that the client is using for communication.
        client_port -> Nullable<Integer>,
        /// Time when this process was started.
        backend_start -> Nullable<Timestamp>,
        /// Time when the current transaction was started.
        xact_start -> Nullable<Timestamp>,
        /// Time when the currently active query was started.
        query_start -> Nullable<Timestamp>,
        /// Time when the state was last changed.
        state_change -> Nullable<Timestamp>,
        /// Type of event the backend is waiting for, if any.
        wait_event_type -> Nullable<Text>,
        /// Wait event name if backend is currently waiting.
        wait_event -> Nullable<Text>,
        /// Current overall state of this backend.
        state -> Nullable<Text>,
        /// Transaction ID of this backend's current transaction.
        backend_xid -> Nullable<Oid>,
        /// Transaction ID of the current backend's transaction snapshot.
        backend_xmin -> Nullable<Oid>,
        /// Identifier of this backend's most recent query.
        query_id -> Nullable<BigInt>,
        /// Text of this backend's most recent query.
        query -> Nullable<Text>,
        /// Type of backend (e.g., client backend, autovacuum worker).
        backend_type -> Nullable<Text>,
    }
}
