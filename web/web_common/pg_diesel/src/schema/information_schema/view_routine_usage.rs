//! `view_routine_usage` view from `information_schema`.

diesel::table! {
    /// `information_schema.view_routine_usage` — view containing one row for each
    /// function or procedure that is referenced in the query expression of a view.
    /// Shows the relationship between views and the underlying routines.
    information_schema.view_routine_usage (table_catalog, table_schema, table_name, specific_catalog, specific_schema, specific_name) {
        /// Name of the database (catalog) containing the view.
        table_catalog -> Nullable<Text>,
        /// Name of the schema containing the view.
        table_schema -> Nullable<Text>,
        /// Name of the view.
        table_name -> Nullable<Text>,
        /// Name of the database (catalog) containing the referenced routine.
        specific_catalog -> Nullable<Text>,
        /// Name of the schema containing the referenced routine.
        specific_schema -> Nullable<Text>,
        /// Name of the referenced routine.
        specific_name -> Nullable<Text>,
    }
}
