//! Submodule for the `pg_catalog.pg_stats` view schema.

diesel::table! {
    /// `pg_catalog.pg_stats` — view providing access to the information stored in pg_statistic.
    /// This view presents the contents of pg_statistic in a more readable format.
    /// Uses `schemaname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stats (schemaname) {
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table.
        tablename -> Nullable<Text>,
        /// Name of the column described by this row.
        attname -> Nullable<Text>,
        /// If true, the stats include values only from child tables.
        inherited -> Nullable<Bool>,
        /// Fraction of column entries that are null.
        null_frac -> Nullable<Float>,
        /// Average width in bytes of column's entries.
        avg_width -> Nullable<Integer>,
        /// Number of distinct nonnull data values in the column.
        n_distinct -> Nullable<Float>,
        /// List of the most common values' frequencies.
        most_common_freqs -> Nullable<Array<Float>>,
        /// Statistical correlation between physical row ordering and logical ordering of the column values.
        correlation -> Nullable<Float>,
        /// List of the frequencies of the most common element values.
        most_common_elem_freqs -> Nullable<Array<Float>>,
        /// Histogram of counts of distinct element values.
        elem_count_histogram -> Nullable<Array<Float>>,
        /// Fraction of range values that are empty.
        range_empty_frac -> Nullable<Float>,
    }
}
