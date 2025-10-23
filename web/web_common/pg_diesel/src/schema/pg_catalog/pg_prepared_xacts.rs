//! Submodule for the `pg_catalog.pg_prepared_xacts` view schema.

diesel::table! {
    /// `pg_catalog.pg_prepared_xacts` — view showing information about prepared transactions.
    /// Each row represents a transaction that has been prepared for two-phase commit.
    /// Uses `gid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_prepared_xacts (gid) {
        /// Transaction ID of the prepared transaction.
        transaction -> Nullable<Oid>,
        /// Global transaction identifier (GID) assigned to the prepared transaction.
        gid -> Nullable<Text>,
        /// Time when the transaction was prepared.
        prepared -> Nullable<Timestamp>,
        /// Name of the user that prepared the transaction.
        owner -> Nullable<Text>,
        /// Name of the database containing the prepared transaction.
        database -> Nullable<Text>,
    }
}
