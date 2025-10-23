//! Submodule for the `information_schema.routine_table_usage` view schema.

diesel::table! {
    /// `information_schema.routine_table_usage` — view containing one row for each
    /// table used by a routine in the current database.
    information_schema.routine_table_usage (specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name, table_catalog, table_schema, table_name) {
        /// Catalog (database) containing the specific routine.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the specific routine.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine.
        specific_name -> Nullable<Text>,
        /// Catalog (database) containing the routine.
        routine_catalog -> Nullable<Text>,
        /// Schema containing the routine.
        routine_schema -> Nullable<Text>,
        /// Name of the routine.
        routine_name -> Nullable<Text>,
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
    }
}
