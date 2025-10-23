//! Schema for pg_catalog.pg_wait_events view.

diesel::table! {
    use diesel::sql_types::*;

    /// Wait events
    ///
    /// The view `pg_wait_events` provides information about wait events that can be monitored.
    pg_catalog.pg_wait_events (wait_type, name) {
        #[sql_name = "type"]
        /// Type of wait event
        wait_type -> Nullable<Text>,
        /// Name of the wait event
        name -> Nullable<Text>,
        /// Description of the wait event
        description -> Nullable<Text>,
    }
}
