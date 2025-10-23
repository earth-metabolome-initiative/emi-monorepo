//! Submodule for the `information_schema.table_constraints` view schema.

diesel::table! {
    /// `information_schema.table_constraints` — view containing one row per table constraint.
    /// Includes primary keys, foreign keys, unique constraints, and check constraints.
    /// Provides metadata such as type, deferrability, enforcement, and null handling.
    information_schema.table_constraints (table_catalog, table_schema, table_name, constraint_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table with the constraint.
        table_catalog -> Text,
        /// Schema containing the table with the constraint.
        table_schema -> Text,
        /// Name of the table containing the constraint.
        table_name -> Text,
        /// Type of the constraint: "PRIMARY KEY", "FOREIGN KEY", "UNIQUE", "CHECK".
        constraint_type -> Text,
        /// "YES" if the constraint is deferrable; "NO" otherwise.
        is_deferrable -> Text,
        /// "YES" if the constraint is initially deferred; "NO" otherwise.
        initially_deferred -> Text,
        /// "YES" if the constraint is enforced, "NO" if it is not.
        enforced -> Text,
        /// "YES" if NULL values are treated as distinct for unique constraints; `NULL` otherwise.
        nulls_distinct -> Nullable<Text>,
    }
}

use super::constraint_column_usage::constraint_column_usage;
diesel::allow_tables_to_appear_in_same_query!(table_constraints, constraint_column_usage);
