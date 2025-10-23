//! Submodule for the `pg_catalog.pg_statio_sys_tables` view schema.

diesel::table! {
    /// `pg_catalog.pg_statio_sys_tables` — view showing I/O statistics for system catalog tables.
    /// Each row represents one system catalog table showing statistics about I/O operations on that table.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_statio_sys_tables (relid) {
        /// OID of this table.
        relid -> Nullable<Oid>,
        /// Name of the schema that this table is in.
        schemaname -> Nullable<Text>,
        /// Name of this table.
        relname -> Nullable<Text>,
        /// Number of disk blocks read from this table.
        heap_blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this table.
        heap_blks_hit -> Nullable<BigInt>,
        /// Number of disk blocks read from all indexes on this table.
        idx_blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in all indexes on this table.
        idx_blks_hit -> Nullable<BigInt>,
        /// Number of disk blocks read from this table's TOAST table.
        toast_blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this table's TOAST table.
        toast_blks_hit -> Nullable<BigInt>,
        /// Number of disk blocks read from this table's TOAST table index.
        tidx_blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this table's TOAST table index.
        tidx_blks_hit -> Nullable<BigInt>,
    }
}
