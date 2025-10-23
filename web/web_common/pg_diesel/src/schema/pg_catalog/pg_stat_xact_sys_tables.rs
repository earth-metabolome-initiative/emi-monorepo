//! Submodule for the `pg_catalog.pg_stat_xact_sys_tables` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_xact_sys_tables` — view showing statistics for system catalog tables in the current transaction.
    /// Each row represents one system catalog table showing statistics about accesses to that table within the current transaction.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_xact_sys_tables (relid) {
        /// OID of this table.
        relid -> Nullable<Oid>,
        /// Name of the schema that this table is in.
        schemaname -> Nullable<Text>,
        /// Name of this table.
        relname -> Nullable<Text>,
        /// Number of sequential scans initiated on this table within the current transaction.
        seq_scan -> Nullable<BigInt>,
        /// Number of live rows fetched by sequential scans within the current transaction.
        seq_tup_read -> Nullable<BigInt>,
        /// Number of index scans initiated on this table within the current transaction.
        idx_scan -> Nullable<BigInt>,
        /// Number of live rows fetched by index scans within the current transaction.
        idx_tup_fetch -> Nullable<BigInt>,
        /// Number of rows inserted within the current transaction.
        n_tup_ins -> Nullable<BigInt>,
        /// Number of rows updated within the current transaction.
        n_tup_upd -> Nullable<BigInt>,
        /// Number of rows deleted within the current transaction.
        n_tup_del -> Nullable<BigInt>,
        /// Number of rows HOT (Heap-Only Tuple) updated within the current transaction.
        n_tup_hot_upd -> Nullable<BigInt>,
        /// Number of rows updated to a new page within the current transaction.
        n_tup_newpage_upd -> Nullable<BigInt>,
    }
}
