//! Submodule for the `information_schema.routine_column_usage` view schema.

diesel::table! {
    /// `information_schema.routine_column_usage` — view containing one row for each
    /// column referenced by a routine in the current database.
    information_schema.routine_column_usage (specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name, table_catalog, table_schema, table_name, column_name) {
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
        /// Name of the column.
        column_name -> Nullable<Text>,
    }
}
