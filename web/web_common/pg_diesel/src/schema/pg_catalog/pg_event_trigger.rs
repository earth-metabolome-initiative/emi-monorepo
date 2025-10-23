//! Submodule for the `pg_catalog.pg_event_trigger` table schema.

diesel::table! {
    /// `pg_catalog.pg_event_trigger` — system catalog containing event triggers.
    /// Event triggers fire on DDL events like CREATE, ALTER, DROP, etc.
    pg_catalog.pg_event_trigger (oid) {
        /// OID of the event trigger.
        oid -> Oid,
        /// Name of the event trigger.
        evtname -> Text,
        /// Event that this trigger fires on (e.g., 'ddl_command_start', 'ddl_command_end').
        evtevent -> Text,
        /// OID of the role that owns the event trigger.
        evtowner -> Oid,
        /// OID of the function to be called when the event occurs.
        evtfoid -> Oid,
        /// Firing mode: 'O' = origin, 'D' = disabled, 'R' = replica, 'A' = always.
        evtenabled -> Text,
        /// Command tags for which this trigger fires (NULL = all commands).
        evttags -> Nullable<Array<Text>>,
    }
}
