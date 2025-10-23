//! Submodule for the `pg_catalog.pg_replication_origin_status` view schema.

diesel::table! {
    /// `pg_catalog.pg_replication_origin_status` — view showing replication origin status.
    /// Each row represents the current state of replication from a particular origin.
    /// Uses `local_id` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_replication_origin_status (local_id) {
        /// Internal OID of the replication origin.
        local_id -> Nullable<Oid>,
        /// External name of the replication origin.
        external_id -> Nullable<Text>,
        /// LSN up to which data from this origin has been replayed.
        remote_lsn -> Nullable<PgLsn>,
        /// Local LSN of the last replayed commit from this origin.
        local_lsn -> Nullable<PgLsn>,
    }
}
