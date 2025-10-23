//! Submodule for the `pg_catalog.pg_replication_slots` view schema.

diesel::table! {
    /// `pg_catalog.pg_replication_slots` — view showing replication slot information.
    /// Each row represents a replication slot that has been created on the server.
    /// Uses `slot_name` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_replication_slots (slot_name) {
        /// Name of the replication slot.
        slot_name -> Nullable<Text>,
        /// Name of the output plugin for logical replication (null for physical slots).
        plugin -> Nullable<Text>,
        /// Type of replication slot: 'physical' or 'logical'.
        slot_type -> Nullable<Text>,
        /// OID of the database this slot is associated with (null for physical slots).
        datoid -> Nullable<Oid>,
        /// Name of the database this slot is associated with.
        database -> Nullable<Text>,
        /// `true` if this is a temporary replication slot.
        temporary -> Nullable<Bool>,
        /// `true` if the slot is currently actively being used.
        active -> Nullable<Bool>,
        /// Process ID of the session using this slot (null if inactive).
        active_pid -> Nullable<Integer>,
        /// Oldest transaction that this slot needs the database to retain.
        xmin -> Nullable<Oid>,
        /// Oldest transaction affecting the system catalogs that this slot needs retained.
        catalog_xmin -> Nullable<Oid>,
        /// WAL position up to which the consumer of this slot has confirmed receiving data.
        restart_lsn -> Nullable<PgLsn>,
        /// LSN up to which the logical slot's consumer has confirmed receiving data.
        confirmed_flush_lsn -> Nullable<PgLsn>,
        /// Status of WAL files required by this slot: 'reserved', 'extended', or 'unreserved'.
        wal_status -> Nullable<Text>,
        /// Number of bytes that can be written to WAL before this slot risks losing data.
        safe_wal_size -> Nullable<BigInt>,
        /// `true` if the slot is enabled for two-phase commit decoding.
        two_phase -> Nullable<Bool>,
        /// Time since the slot became inactive (null if active).
        inactive_since -> Nullable<Timestamp>,
        /// `true` if the slot has been invalidated due to conflicts.
        conflicting -> Nullable<Bool>,
        /// Reason for invalidation (null if not invalidated).
        invalidation_reason -> Nullable<Text>,
        /// `true` if the slot is enabled for failover.
        failover -> Nullable<Bool>,
        /// `true` if the slot is synchronized from a primary server.
        synced -> Nullable<Bool>,
    }
}
