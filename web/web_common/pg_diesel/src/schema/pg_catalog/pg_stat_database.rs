//! Submodule for the `pg_catalog.pg_stat_database` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_database` — view showing database-wide statistics.
    /// Each row represents one database showing statistics about accesses to that specific database.
    /// Uses `datid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_database (datid) {
        /// OID of this database.
        datid -> Nullable<Oid>,
        /// Name of this database.
        datname -> Nullable<Text>,
        /// Number of backends currently connected to this database.
        numbackends -> Nullable<Integer>,
        /// Number of transactions committed in this database.
        xact_commit -> Nullable<BigInt>,
        /// Number of transactions rolled back in this database.
        xact_rollback -> Nullable<BigInt>,
        /// Number of disk blocks read in this database.
        blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this database.
        blks_hit -> Nullable<BigInt>,
        /// Number of rows returned by queries in this database.
        tup_returned -> Nullable<BigInt>,
        /// Number of rows fetched by queries in this database.
        tup_fetched -> Nullable<BigInt>,
        /// Number of rows inserted in this database.
        tup_inserted -> Nullable<BigInt>,
        /// Number of rows updated in this database.
        tup_updated -> Nullable<BigInt>,
        /// Number of rows deleted in this database.
        tup_deleted -> Nullable<BigInt>,
        /// Number of queries canceled due to conflicts in this database.
        conflicts -> Nullable<BigInt>,
        /// Number of temporary files created by queries in this database.
        temp_files -> Nullable<BigInt>,
        /// Total amount of data written to temporary files by queries in this database.
        temp_bytes -> Nullable<BigInt>,
        /// Number of deadlocks detected in this database.
        deadlocks -> Nullable<BigInt>,
        /// Number of data page checksum failures detected in this database.
        checksum_failures -> Nullable<BigInt>,
        /// Time of the last checksum failure in this database.
        checksum_last_failure -> Nullable<Timestamp>,
        /// Time spent reading data file blocks by backends in this database, in milliseconds.
        blk_read_time -> Nullable<Double>,
        /// Time spent writing data file blocks by backends in this database, in milliseconds.
        blk_write_time -> Nullable<Double>,
        /// Time spent by database sessions in this database, in milliseconds.
        session_time -> Nullable<Double>,
        /// Time spent executing SQL statements in this database, in milliseconds.
        active_time -> Nullable<Double>,
        /// Time spent idle in transactions in this database, in milliseconds.
        idle_in_transaction_time -> Nullable<Double>,
        /// Total number of sessions established to this database.
        sessions -> Nullable<BigInt>,
        /// Number of database sessions that were terminated because connection was lost.
        sessions_abandoned -> Nullable<BigInt>,
        /// Number of database sessions that were terminated by fatal errors.
        sessions_fatal -> Nullable<BigInt>,
        /// Number of database sessions that were terminated by operator intervention.
        sessions_killed -> Nullable<BigInt>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
