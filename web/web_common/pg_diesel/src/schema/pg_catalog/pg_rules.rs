//! Submodule for the `pg_catalog.pg_rules` view schema.

diesel::table! {
    /// `pg_catalog.pg_rules` — view showing information about rules.
    /// Each row represents a rule defined on a table or view in a user-friendly format.
    /// Uses `rulename` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_rules (rulename) {
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table the rule is defined on.
        tablename -> Nullable<Text>,
        /// Name of the rule.
        rulename -> Nullable<Text>,
        /// SQL definition of the rule.
        definition -> Nullable<Text>,
    }
}
