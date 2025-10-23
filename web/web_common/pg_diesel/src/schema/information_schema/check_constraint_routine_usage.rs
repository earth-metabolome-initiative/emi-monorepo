//! Submodule for the `information_schema.check_constraint_routine_usage` view
//! schema.

diesel::table! {
    /// `information_schema.check_constraint_routine_usage` — view containing one row for each
    /// routine (function or procedure) that is used in a check constraint. This view establishes
    /// the dependency relationship between check constraints and the functions they call.
    information_schema.check_constraint_routine_usage (constraint_catalog, constraint_schema, constraint_name, specific_catalog, specific_schema, specific_name) {
        /// Catalog (database) containing the check constraint.
        constraint_catalog -> Nullable<Text>,
        /// Schema containing the check constraint.
        constraint_schema -> Nullable<Text>,
        /// Name of the check constraint.
        constraint_name -> Nullable<Text>,
        /// Catalog (database) containing the routine used by the constraint.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the routine used by the constraint.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine used by the constraint.
        specific_name -> Nullable<Text>,
    }
}
