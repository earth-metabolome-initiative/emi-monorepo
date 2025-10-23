//! `view_table_usage` view from `information_schema`.

diesel::table! {
    /// `information_schema.view_table_usage` — view containing one row for each
    /// table that is referenced in the query expression of a view.
    /// Shows the relationship between views and the underlying tables.
    information_schema.view_table_usage (view_catalog, view_schema, view_name, table_catalog, table_schema, table_name) {
        /// Name of the database (catalog) containing the view.
        view_catalog -> Nullable<Text>,
        /// Name of the schema containing the view.
        view_schema -> Nullable<Text>,
        /// Name of the view.
        view_name -> Nullable<Text>,
        /// Name of the database (catalog) containing the referenced table.
        table_catalog -> Nullable<Text>,
        /// Name of the schema containing the referenced table.
        table_schema -> Nullable<Text>,
        /// Name of the referenced table.
        table_name -> Nullable<Text>,
    }
}
