//! `triggered_update_columns` view from `information_schema`.

diesel::table! {
    /// `information_schema.triggered_update_columns` — view containing information
    /// about columns that are used by triggers for UPDATE events. Contains one row
    /// for each column that can trigger an UPDATE trigger.
    information_schema.triggered_update_columns (trigger_catalog, trigger_schema, trigger_name, event_object_catalog, event_object_schema, event_object_table, event_object_column) {
        /// Name of the database (catalog) containing the trigger.
        trigger_catalog -> Nullable<Text>,
        /// Name of the schema containing the trigger.
        trigger_schema -> Nullable<Text>,
        /// Name of the trigger.
        trigger_name -> Nullable<Text>,
        /// Name of the database (catalog) containing the table with the trigger.
        event_object_catalog -> Nullable<Text>,
        /// Name of the schema containing the table with the trigger.
        event_object_schema -> Nullable<Text>,
        /// Name of the table that has the trigger.
        event_object_table -> Nullable<Text>,
        /// Name of the column that can trigger the UPDATE trigger.
        event_object_column -> Nullable<Text>,
    }
}
