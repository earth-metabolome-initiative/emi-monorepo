//! Submodule for the `pg_catalog.pg_rewrite` table schema.

diesel::table! {
    /// `pg_catalog.pg_rewrite` — system catalog containing rewrite rules.
    /// Each row represents a rewrite rule defined on a table or view.
    pg_catalog.pg_rewrite (oid) {
        /// OID of the rewrite rule.
        oid -> Oid,
        /// Name of the rule.
        rulename -> Text,
        /// OID of the table/view this rule is for (references pg_class).
        ev_class -> Oid,
        /// Event type: '1' (SELECT), '2' (UPDATE), '3' (INSERT), '4' (DELETE).
        ev_type -> Text,
        /// Rule firing mode: 'O' (origin), 'D' (disabled), 'R' (replica), 'A' (always).
        ev_enabled -> Text,
        /// `true` if the rule is an INSTEAD rule.
        is_instead -> Bool,
        /// Expression tree for the rule's WHERE condition.
        ev_qual -> Text,
        /// Expression tree for the rule's action.
        ev_action -> Text,
    }
}
