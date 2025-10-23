//! Submodule for the `pg_catalog.pg_sequences` view schema.

diesel::table! {
    /// `pg_catalog.pg_sequences` — view showing information about sequences.
    /// Each row represents a sequence with its configuration in a user-friendly format.
    /// Uses `sequencename` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_sequences (sequencename) {
        /// Name of the schema containing the sequence.
        schemaname -> Nullable<Text>,
        /// Name of the sequence.
        sequencename -> Nullable<Text>,
        /// Name of the sequence owner.
        sequenceowner -> Nullable<Text>,
        /// OID of the data type of the sequence.
        data_type -> Nullable<Oid>,
        /// Start value of the sequence.
        start_value -> Nullable<BigInt>,
        /// Minimum value of the sequence.
        min_value -> Nullable<BigInt>,
        /// Maximum value of the sequence.
        max_value -> Nullable<BigInt>,
        /// Increment value.
        increment_by -> Nullable<BigInt>,
        /// Whether the sequence cycles.
        cycle -> Nullable<Bool>,
        /// Cache size.
        cache_size -> Nullable<BigInt>,
        /// Last value returned by the sequence.
        last_value -> Nullable<BigInt>,
    }
}
