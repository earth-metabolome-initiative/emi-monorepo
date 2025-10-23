//! Submodule for the `information_schema.constraint_column_usage` view schema.

diesel::table! {
    /// `information_schema.constraint_column_usage` — view containing one row per column
    /// that is used by a table constraint (primary key, unique, foreign key, or check).
    /// Links each constrained column to its constraint metadata.
    information_schema.constraint_column_usage (table_catalog, table_schema, table_name, column_name) {
        /// Catalog (database) containing the constraint.
        constraint_catalog -> Text,
        /// Schema containing the constraint.
        constraint_schema -> Text,
        /// Name of the constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the table with the constrained column.
        table_catalog -> Text,
        /// Schema containing the table with the constrained column.
        table_schema -> Text,
        /// Name of the table containing the constrained column.
        table_name -> Text,
        /// Name of the constrained column.
        column_name -> Text,
    }
}

use super::key_column_usage::key_column_usage;
diesel::allow_tables_to_appear_in_same_query!(constraint_column_usage, key_column_usage);

use super::tables::tables;
diesel::allow_tables_to_appear_in_same_query!(constraint_column_usage, tables);
