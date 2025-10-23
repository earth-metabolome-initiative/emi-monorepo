//! `triggers` view from `information_schema`.

diesel::table! {
    /// `information_schema.triggers` — view containing one row for each trigger
    /// in the current database that the current user owns or has some privilege on.
    /// Provides comprehensive metadata about triggers including timing, events, and actions.
    information_schema.triggers (trigger_catalog, trigger_schema, trigger_name) {
        /// Name of the database (catalog) containing the trigger.
        trigger_catalog -> Nullable<Text>,
        /// Name of the schema containing the trigger.
        trigger_schema -> Nullable<Text>,
        /// Name of the trigger.
        trigger_name -> Nullable<Text>,
        /// Event that activates the trigger (INSERT, UPDATE, DELETE, TRUNCATE).
        event_manipulation -> Nullable<Text>,
        /// Name of the database (catalog) containing the table with the trigger.
        event_object_catalog -> Nullable<Text>,
        /// Name of the schema containing the table with the trigger.
        event_object_schema -> Nullable<Text>,
        /// Name of the table that has the trigger.
        event_object_table -> Nullable<Text>,
        /// Order of execution when multiple triggers fire on the same event.
        action_order -> Nullable<Integer>,
        /// Condition that must be met for the trigger to fire (WHEN clause).
        action_condition -> Nullable<Text>,
        /// SQL statement(s) executed when the trigger fires.
        action_statement -> Nullable<Text>,
        /// Orientation of the trigger (ROW or STATEMENT).
        action_orientation -> Nullable<Text>,
        /// Timing of the trigger execution (BEFORE, AFTER, INSTEAD OF).
        action_timing -> Nullable<Text>,
        /// Name of the transition table for old row values.
        action_reference_old_table -> Nullable<Text>,
        /// Name of the transition table for new row values.
        action_reference_new_table -> Nullable<Text>,
        /// Name of the identifier for old row values.
        action_reference_old_row -> Nullable<Text>,
        /// Name of the identifier for new row values.
        action_reference_new_row -> Nullable<Text>,
        /// Timestamp when the trigger was created.
        created -> Nullable<Timestamp>,
    }
}
