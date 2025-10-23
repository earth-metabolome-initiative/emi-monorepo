//! `view_column_usage` view from `information_schema`.

diesel::table! {
    /// `information_schema.view_column_usage` — view containing one row for each
    /// column that is referenced in the query expression of a view.
    /// Shows the relationship between views and the underlying table columns.
    information_schema.view_column_usage (view_catalog, view_schema, view_name, table_catalog, table_schema, table_name, column_name) {
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
        /// Name of the referenced column.
        column_name -> Nullable<Text>,
    }
}
