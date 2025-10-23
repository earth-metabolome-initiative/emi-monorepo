//! Submodule for the `information_schema.check_constraints` view schema.

diesel::table! {
    /// `information_schema.check_constraints` — view containing one row per check constraint.
    /// Provides metadata about check constraints defined on tables, including the expression that must hold.
    information_schema.check_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the check constraint.
        constraint_catalog -> Text,
        /// Schema containing the check constraint.
        constraint_schema -> Text,
        /// Name of the check constraint.
        constraint_name -> Text,
        /// The Boolean expression that defines the check constraint.
        check_clause -> Text,
    }
}

use super::table_constraints::table_constraints;
diesel::allow_tables_to_appear_in_same_query!(check_constraints, table_constraints);

use super::constraint_column_usage::constraint_column_usage;
diesel::allow_tables_to_appear_in_same_query!(check_constraints, constraint_column_usage);

use super::columns::columns;
diesel::allow_tables_to_appear_in_same_query!(check_constraints, columns);
