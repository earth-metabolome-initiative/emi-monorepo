//! Submodule for the `pg_catalog.pg_prepared_statements` view schema.

diesel::table! {
    /// `pg_catalog.pg_prepared_statements` — view showing all prepared statements in the current session.
    /// Each row represents a prepared statement with its SQL text and metadata.
    /// Uses `name` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_prepared_statements (name) {
        /// Name of the prepared statement.
        name -> Nullable<Text>,
        /// SQL text of the prepared statement.
        statement -> Nullable<Text>,
        /// Time when the statement was prepared.
        prepare_time -> Nullable<Timestamp>,
        /// OIDs of the parameter types.
        parameter_types -> Nullable<Array<Oid>>,
        /// OIDs of the result column types.
        result_types -> Nullable<Array<Oid>>,
        /// `true` if the prepared statement was created via SQL PREPARE command.
        from_sql -> Nullable<Bool>,
        /// Number of times generic plans have been chosen.
        generic_plans -> Nullable<BigInt>,
        /// Number of times custom plans have been chosen.
        custom_plans -> Nullable<BigInt>,
    }
}
