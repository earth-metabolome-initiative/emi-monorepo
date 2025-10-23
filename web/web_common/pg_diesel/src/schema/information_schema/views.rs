//! `views` view from `information_schema`.

diesel::table! {
    /// `information_schema.views` — view containing one row for each table that is actually a view.
    /// Contains metadata about all database views including their definitions and capabilities.
    information_schema.views (table_catalog, table_schema, table_name) {
        /// Name of the database (catalog) containing the view.
        table_catalog -> Nullable<Text>,
        /// Name of the schema containing the view.
        table_schema -> Nullable<Text>,
        /// Name of the view.
        table_name -> Nullable<Text>,
        /// Query expression defining the view (SQL text).
        view_definition -> Nullable<Text>,
        /// Whether the view has a CHECK OPTION defined.
        check_option -> Nullable<Text>,
        /// Whether the view is updatable (can accept UPDATE statements).
        is_updatable -> Nullable<Text>,
        /// Whether the view allows INSERT operations.
        is_insertable_into -> Nullable<Text>,
        /// Whether the view supports trigger-based updates.
        is_trigger_updatable -> Nullable<Text>,
        /// Whether the view supports trigger-based deletes.
        is_trigger_deletable -> Nullable<Text>,
        /// Whether the view supports trigger-based inserts.
        is_trigger_insertable_into -> Nullable<Text>,
    }
}
