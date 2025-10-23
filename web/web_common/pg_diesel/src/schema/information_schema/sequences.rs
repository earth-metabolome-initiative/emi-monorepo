//! `sequences` view from `information_schema`.

diesel::table! {
    /// `information_schema.sequences` — view containing one row for each sequence
    /// in the current database that is accessible to the current user.
    /// Provides metadata about sequences including data type, precision, range,
    /// increment, and cycling behavior.
    information_schema.sequences (sequence_catalog, sequence_schema, sequence_name) {
        /// Name of the database (catalog) containing the sequence.
        sequence_catalog -> Nullable<Text>,
        /// Name of the schema containing the sequence.
        sequence_schema -> Nullable<Text>,
        /// Name of the sequence.
        sequence_name -> Nullable<Text>,
        /// Data type of the sequence (typically "bigint", "integer", or "smallint").
        data_type -> Nullable<Text>,
        /// Precision of the numeric data type (number of significant digits).
        numeric_precision -> Nullable<Integer>,
        /// Radix of the numeric precision (typically 2 or 10).
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale of the numeric data type (digits after decimal point).
        numeric_scale -> Nullable<Integer>,
        /// Start value of the sequence.
        start_value -> Nullable<Text>,
        /// Minimum value of the sequence.
        minimum_value -> Nullable<Text>,
        /// Maximum value of the sequence.
        maximum_value -> Nullable<Text>,
        /// Increment value of the sequence.
        increment -> Nullable<Text>,
        /// "YES" if the sequence cycles when it reaches its limit, "NO" otherwise.
        cycle_option -> Nullable<Text>,
    }
}
