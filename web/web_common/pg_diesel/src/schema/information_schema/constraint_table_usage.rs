//! Submodule for the `information_schema.constraint_table_usage` view schema.

diesel::table! {
    /// `information_schema.constraint_table_usage` — view containing one row per table
    /// that is referenced by a table constraint (primary key, unique, foreign key, or check).
    /// Provides a high-level mapping from constraints to the tables they involve.
    information_schema.constraint_table_usage (table_catalog, table_schema, table_name, constraint_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table referenced by the constraint.
        table_catalog -> Text,
        /// Schema containing the table referenced by the constraint.
        table_schema -> Text,
        /// Name of the table referenced by the constraint.
        table_name -> Text,
    }
}
