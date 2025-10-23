//! Submodule for the `pg_catalog.pg_subscription` table schema.

diesel::table! {
    /// `pg_catalog.pg_subscription` — table storing logical replication subscriptions.
    /// Each row represents a subscription for a database.
    /// Uses `oid` as the primary key.
    pg_catalog.pg_subscription (oid) {
        /// Row identifier.
        oid -> Oid,
        /// OID of the database that the subscription resides in.
        subdbid -> Oid,
        /// Finish LSN of the transaction whose changes are to be skipped.
        subskiplsn -> PgLsn,
        /// Name of the subscription.
        subname -> Text,
        /// Owner of the subscription.
        subowner -> Oid,
        /// If true, the subscription is enabled and should be replicating.
        subenabled -> Bool,
        /// If true, the subscription will request that the publisher send data in binary format.
        subbinary -> Bool,
        /// Controls how to handle the streaming of in-progress transactions.
        substream -> Text,
        /// State of two-phase commit for this subscription.
        subtwophasestate -> Text,
        /// If true, the subscription will be disabled if any errors are detected by subscription workers.
        subdisableonerr -> Bool,
        /// If true, the subscription will request that the publisher send a password.
        subpasswordrequired -> Bool,
        /// If true, the subscription will run as the subscription owner.
        subrunasowner -> Bool,
        /// If true, the subscription will request that the publisher enable failover support.
        subfailover -> Bool,
        /// Connection string to the publisher.
        subconninfo -> Text,
        /// Name of the replication slot on the publisher.
        subslotname -> Nullable<Text>,
        /// Synchronous commit setting for the subscription's apply worker.
        subsynccommit -> Text,
        /// Array of subscribed publication names.
        subpublications -> Array<Text>,
        /// Origin name for this subscription.
        suborigin -> Nullable<Text>,
    }
}
