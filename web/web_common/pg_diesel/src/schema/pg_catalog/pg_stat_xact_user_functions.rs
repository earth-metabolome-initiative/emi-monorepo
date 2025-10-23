//! Submodule for the `pg_catalog.pg_stat_xact_user_functions` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_xact_user_functions` — view showing statistics for user-defined functions in the current transaction.
    /// Each row represents one tracked function showing statistics about calls to that function within the current transaction.
    /// Uses `funcid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_xact_user_functions (funcid) {
        /// OID of this function.
        funcid -> Nullable<Oid>,
        /// Name of the schema that this function is in.
        schemaname -> Nullable<Text>,
        /// Name of this function.
        funcname -> Nullable<Text>,
        /// Number of times this function has been called within the current transaction.
        calls -> Nullable<BigInt>,
        /// Total time spent in this function and all other functions called by it within the current transaction, in milliseconds.
        total_time -> Nullable<Double>,
        /// Total time spent in this function itself, not including other functions called by it, within the current transaction, in milliseconds.
        self_time -> Nullable<Double>,
    }
}
