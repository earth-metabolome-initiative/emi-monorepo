//! Submodule for the `pg_catalog.pg_stats_ext_exprs` view schema.

diesel::table! {
    /// `pg_catalog.pg_stats_ext_exprs` — view providing access to statistics on expressions in extended statistics.
    /// This view presents statistics for each expression in extended statistics objects.
    /// Uses `schemaname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stats_ext_exprs (schemaname) {
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table.
        tablename -> Nullable<Text>,
        /// Name of the schema containing the statistics object.
        statistics_schemaname -> Nullable<Text>,
        /// Name of the statistics object.
        statistics_name -> Nullable<Text>,
        /// Owner of the statistics object.
        statistics_owner -> Nullable<Text>,
        /// Expression text.
        expr -> Nullable<Text>,
        /// If true, the stats include values only from child tables.
        inherited -> Nullable<Bool>,
        /// Fraction of expression values that are null.
        null_frac -> Nullable<Float>,
        /// Average width in bytes of expression values.
        avg_width -> Nullable<Integer>,
        /// Number of distinct nonnull expression values.
        n_distinct -> Nullable<Float>,
        /// List of the most common values' frequencies.
        most_common_freqs -> Nullable<Array<Float>>,
        /// Statistical correlation between physical row ordering and logical ordering of the expression values.
        correlation -> Nullable<Float>,
        /// List of the frequencies of the most common element values.
        most_common_elem_freqs -> Nullable<Array<Float>>,
        /// Histogram of counts of distinct element values.
        elem_count_histogram -> Nullable<Array<Float>>,
    }
}
