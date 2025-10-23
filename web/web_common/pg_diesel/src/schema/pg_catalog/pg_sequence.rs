//! Submodule for the `pg_catalog.pg_sequence` table schema.

diesel::table! {
    /// `pg_catalog.pg_sequence` — system catalog containing sequence parameters.
    /// Each row represents a sequence object with its configuration.
    pg_catalog.pg_sequence (seqrelid) {
        /// OID of the sequence (references pg_class).
        seqrelid -> Oid,
        /// OID of the data type of the sequence.
        seqtypid -> Oid,
        /// Start value of the sequence.
        seqstart -> BigInt,
        /// Increment value (can be negative).
        seqincrement -> BigInt,
        /// Maximum value of the sequence.
        seqmax -> BigInt,
        /// Minimum value of the sequence.
        seqmin -> BigInt,
        /// Cache size (number of values to preallocate).
        seqcache -> BigInt,
        /// `true` if the sequence cycles when it reaches max/min.
        seqcycle -> Bool,
    }
}
