//! Submodule for the `pg_catalog.pg_stat_progress_copy` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_copy` — view showing COPY command progress.
    /// Each row represents one backend running COPY showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_copy (pid) {
        /// Process ID of the backend running COPY.
        pid -> Nullable<Integer>,
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// OID of the table being copied.
        relid -> Nullable<Oid>,
        /// Command that is running (COPY FROM or COPY TO).
        command -> Nullable<Text>,
        /// Type of entity being copied (TABLE or PROGRAM).
        r#type -> Nullable<Text>,
        /// Number of bytes processed.
        bytes_processed -> Nullable<BigInt>,
        /// Total number of bytes (for COPY FROM with file input).
        bytes_total -> Nullable<BigInt>,
        /// Number of tuples processed.
        tuples_processed -> Nullable<BigInt>,
        /// Number of tuples excluded by WHERE clause.
        tuples_excluded -> Nullable<BigInt>,
        /// Number of tuples skipped.
        tuples_skipped -> Nullable<BigInt>,
    }
}
