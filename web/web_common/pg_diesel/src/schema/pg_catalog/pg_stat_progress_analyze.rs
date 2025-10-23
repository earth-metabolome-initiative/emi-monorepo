//! Submodule for the `pg_catalog.pg_stat_progress_analyze` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_progress_analyze` — view showing ANALYZE command progress.
    /// Each row represents one backend running ANALYZE showing progress information.
    /// Uses `pid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_progress_analyze (pid) {
        /// Process ID of the backend running ANALYZE.
        pid -> Nullable<Integer>,
        /// OID of the database this backend is connected to.
        datid -> Nullable<Oid>,
        /// Name of the database this backend is connected to.
        datname -> Nullable<Text>,
        /// OID of the table being analyzed.
        relid -> Nullable<Oid>,
        /// Current processing phase.
        phase -> Nullable<Text>,
        /// Total number of sample blocks to be scanned.
        sample_blks_total -> Nullable<BigInt>,
        /// Number of sample blocks scanned.
        sample_blks_scanned -> Nullable<BigInt>,
        /// Total number of extended statistics to compute.
        ext_stats_total -> Nullable<BigInt>,
        /// Number of extended statistics computed.
        ext_stats_computed -> Nullable<BigInt>,
        /// Total number of child tables to process.
        child_tables_total -> Nullable<BigInt>,
        /// Number of child tables processed.
        child_tables_done -> Nullable<BigInt>,
        /// OID of the child table currently being analyzed.
        current_child_table_relid -> Nullable<Oid>,
    }
}
