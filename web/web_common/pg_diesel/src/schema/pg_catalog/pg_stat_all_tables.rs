//! Submodule for the `pg_catalog.pg_stat_all_tables` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_all_tables` — view showing statistics for all tables.
    /// Each row represents one table showing statistics about accesses to that specific table.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_all_tables (relid) {
        /// OID of this table.
        relid -> Nullable<Oid>,
        /// Name of the schema that this table is in.
        schemaname -> Nullable<Text>,
        /// Name of this table.
        relname -> Nullable<Text>,
        /// Number of sequential scans initiated on this table.
        seq_scan -> Nullable<BigInt>,
        /// Time of the last sequential scan on this table.
        last_seq_scan -> Nullable<Timestamp>,
        /// Number of live rows fetched by sequential scans.
        seq_tup_read -> Nullable<BigInt>,
        /// Number of index scans initiated on this table.
        idx_scan -> Nullable<BigInt>,
        /// Time of the last index scan on this table.
        last_idx_scan -> Nullable<Timestamp>,
        /// Number of live rows fetched by index scans.
        idx_tup_fetch -> Nullable<BigInt>,
        /// Number of rows inserted.
        n_tup_ins -> Nullable<BigInt>,
        /// Number of rows updated.
        n_tup_upd -> Nullable<BigInt>,
        /// Number of rows deleted.
        n_tup_del -> Nullable<BigInt>,
        /// Number of rows HOT (Heap-Only Tuple) updated.
        n_tup_hot_upd -> Nullable<BigInt>,
        /// Number of rows updated to a new page.
        n_tup_newpage_upd -> Nullable<BigInt>,
        /// Estimated number of live rows.
        n_live_tup -> Nullable<BigInt>,
        /// Estimated number of dead rows.
        n_dead_tup -> Nullable<BigInt>,
        /// Estimated number of rows modified since last analyze.
        n_mod_since_analyze -> Nullable<BigInt>,
        /// Estimated number of rows inserted since last vacuum.
        n_ins_since_vacuum -> Nullable<BigInt>,
        /// Time of the last manual vacuum on this table.
        last_vacuum -> Nullable<Timestamp>,
        /// Time of the last autovacuum on this table.
        last_autovacuum -> Nullable<Timestamp>,
        /// Time of the last manual analyze on this table.
        last_analyze -> Nullable<Timestamp>,
        /// Time of the last autoanalyze on this table.
        last_autoanalyze -> Nullable<Timestamp>,
        /// Number of times this table has been manually vacuumed.
        vacuum_count -> Nullable<BigInt>,
        /// Number of times this table has been autovacuumed.
        autovacuum_count -> Nullable<BigInt>,
        /// Number of times this table has been manually analyzed.
        analyze_count -> Nullable<BigInt>,
        /// Number of times this table has been autoanalyzed.
        autoanalyze_count -> Nullable<BigInt>,
    }
}
