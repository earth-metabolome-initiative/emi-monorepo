//! Submodule for the `pg_catalog.pg_stat_archiver` view schema.

diesel::table! {
    /// `pg_catalog.pg_stat_archiver` — view showing WAL archiver statistics.
    /// Contains a single row showing cluster-wide WAL archiver process statistics.
    /// Uses `archived_count` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_stat_archiver (archived_count) {
        /// Number of WAL files that have been successfully archived.
        archived_count -> Nullable<BigInt>,
        /// Name of the last WAL file successfully archived.
        last_archived_wal -> Nullable<Text>,
        /// Time of the last successful WAL file archival.
        last_archived_time -> Nullable<Timestamp>,
        /// Number of failed attempts for archiving WAL files.
        failed_count -> Nullable<BigInt>,
        /// Name of the last WAL file that failed to be archived.
        last_failed_wal -> Nullable<Text>,
        /// Time of the last failed WAL file archival.
        last_failed_time -> Nullable<Timestamp>,
        /// Time at which these statistics were last reset.
        stats_reset -> Nullable<Timestamp>,
    }
}
