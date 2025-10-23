//! Submodule for the `pg_catalog.pg_stat_progress_cluster` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_cluster` — view showing CLUSTER and VACUUM FULL command progress.
    /// Each row represents one backend running CLUSTER or VACUUM FULL showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_cluster (pid) {
        /// Process ID of the backend running CLUSTER or VACUUM FULL.
        pid -> Nullable<Integer>,
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// OID of the table being clustered.
        relid -> Nullable<Oid>,
        /// Command that is running (CLUSTER or VACUUM FULL).
        command -> Nullable<Text>,
        /// Current processing phase.
        phase -> Nullable<Text>,
        /// OID of the index being used for clustering.
        cluster_index_relid -> Nullable<Oid>,
        /// Number of heap tuples scanned.
        heap_tuples_scanned -> Nullable<BigInt>,
        /// Number of heap tuples written.
        heap_tuples_written -> Nullable<BigInt>,
        /// Total number of heap blocks in the table.
        heap_blks_total -> Nullable<BigInt>,
        /// Number of heap blocks scanned.
        heap_blks_scanned -> Nullable<BigInt>,
        /// Number of indexes rebuilt.
        index_rebuild_count -> Nullable<BigInt>,
    }
}
