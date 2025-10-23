//! Submodule for the `pg_catalog.pg_statio_user_sequences` view schema.

diesel::table! {
    /// `pg_catalog.pg_statio_user_sequences` — view showing I/O statistics for user-defined sequences.
    /// Each row represents one user-defined sequence showing statistics about I/O operations on that sequence.
    /// Uses `relid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_statio_user_sequences (relid) {
        /// OID of this sequence.
        relid -> Nullable<Oid>,
        /// Name of the schema that this sequence is in.
        schemaname -> Nullable<Text>,
        /// Name of this sequence.
        relname -> Nullable<Text>,
        /// Number of disk blocks read from this sequence.
        blks_read -> Nullable<BigInt>,
        /// Number of buffer hits in this sequence.
        blks_hit -> Nullable<BigInt>,
    }
}
