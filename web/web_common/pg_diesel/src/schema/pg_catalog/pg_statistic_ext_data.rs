//! Submodule for the `pg_catalog.pg_statistic_ext_data` table schema.

diesel::table! {
    /// `pg_catalog.pg_statistic_ext_data` — table storing data for extended statistics objects.
    /// Each row represents statistics data for an extended statistics object.
    /// Uses composite primary key (stxoid, stxdinherit).
    pg_catalog.pg_statistic_ext_data (stxoid, stxdinherit) {
        /// The extended statistics object containing the definition for this data.
        stxoid -> Oid,
        /// If true, the stats include values only from child tables, not the named table.
        stxdinherit -> Bool,
    }
}
