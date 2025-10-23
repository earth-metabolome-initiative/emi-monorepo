//! Submodule for the `pg_catalog.pg_subscription_rel` table schema.

diesel::table! {
    /// `pg_catalog.pg_subscription_rel` — table storing the state for each replicated relation in a subscription.
    /// Each row represents a relation in a subscription.
    /// Uses composite primary key (srrelid, srsubid).
    pg_catalog.pg_subscription_rel (srrelid, srsubid) {
        /// Reference to the subscription.
        srsubid -> Oid,
        /// Reference to the relation.
        srrelid -> Oid,
        /// State code for the relation replication.
        srsubstate -> Text,
        /// Remote LSN of the state change used for synchronization coordination.
        srsublsn -> Nullable<PgLsn>,
    }
}
