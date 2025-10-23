//! Submodule for the `information_schema.routine_routine_usage` view schema.

diesel::table! {
    /// `information_schema.routine_routine_usage` — view containing one row for each
    /// routine that is used by another routine in the current database.
    information_schema.routine_routine_usage (specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name) {
        /// Catalog (database) containing the specific routine.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the specific routine.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine.
        specific_name -> Nullable<Text>,
        /// Catalog (database) containing the used routine.
        routine_catalog -> Nullable<Text>,
        /// Schema containing the used routine.
        routine_schema -> Nullable<Text>,
        /// Name of the used routine.
        routine_name -> Nullable<Text>,
    }
}
