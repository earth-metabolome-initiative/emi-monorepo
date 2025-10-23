//! Submodule for the `information_schema.key_column_usage` view schema.

diesel::table! {
    /// `information_schema.key_column_usage` — view containing one row per column
    /// that is constrained by a primary key, unique key, or foreign key.
    /// Provides metadata linking columns to their constraints and, for foreign keys,
    /// the referenced unique/primary key column.
    information_schema.key_column_usage (table_catalog, table_schema, table_name, column_name) {
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
        /// Position of the column within the constraint (1-based).
        ordinal_position -> Integer,
        /// For foreign key constraints: position of this column within the referenced unique or primary key;
        /// `NULL` for primary key or unique constraints.
        position_in_unique_constraint -> Nullable<Integer>,
    }
}

use super::table_constraints::table_constraints;
diesel::allow_tables_to_appear_in_same_query!(key_column_usage, table_constraints);

use super::referential_constraints::referential_constraints;
diesel::allow_tables_to_appear_in_same_query!(key_column_usage, referential_constraints);

use super::tables::tables;
diesel::allow_tables_to_appear_in_same_query!(key_column_usage, tables);
