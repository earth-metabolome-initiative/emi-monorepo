//! Submodule for the `pg_catalog.pg_stat_database_conflicts` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_database_conflicts` — view showing database conflicts on standby servers.
    /// Each row represents one database showing the number of queries canceled due to various conflicts.
    /// Uses `datid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_database_conflicts (datid) {
        /// OID of this database.
        datid -> Nullable<Oid>,
        /// Name of this database.
        datname -> Nullable<Text>,
        /// Number of queries canceled due to dropped tablespaces.
        confl_tablespace -> Nullable<BigInt>,
        /// Number of queries canceled due to lock timeouts.
        confl_lock -> Nullable<BigInt>,
        /// Number of queries canceled due to old snapshots.
        confl_snapshot -> Nullable<BigInt>,
        /// Number of queries canceled due to pinned buffers.
        confl_bufferpin -> Nullable<BigInt>,
        /// Number of queries canceled due to deadlocks.
        confl_deadlock -> Nullable<BigInt>,
        /// Number of queries canceled due to active logical replication slots.
        confl_active_logicalslot -> Nullable<BigInt>,
    }
}
