//! Submodule for the `pg_catalog.pg_stats_ext` view schema.

diesel::table! {
    /// `pg_catalog.pg_stats_ext` — view providing access to extended statistics.
    /// This view presents the contents of pg_statistic_ext and pg_statistic_ext_data in a more readable format.
    /// Uses `schemaname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stats_ext (schemaname) {
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
        /// Names of the columns included in the statistics.
        attnames -> Nullable<Array<Text>>,
        /// Expressions included in the statistics.
        exprs -> Nullable<Array<Text>>,
        /// Types of statistics that are enabled.
        kinds -> Nullable<Array<Text>>,
        /// If true, the stats include values only from child tables.
        inherited -> Nullable<Bool>,
        /// Most common combinations of values.
        most_common_vals -> Nullable<Array<Text>>,
        /// Flags showing which of the most common values are null.
        most_common_val_nulls -> Nullable<Array<Bool>>,
        /// Frequencies of the most common combinations.
        most_common_freqs -> Nullable<Array<Double>>,
        /// Base frequencies (without dependencies) of the most common combinations.
        most_common_base_freqs -> Nullable<Array<Double>>,
    }
}
