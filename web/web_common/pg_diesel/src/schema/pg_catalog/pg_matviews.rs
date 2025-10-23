//! Submodule for the `pg_catalog.pg_matviews` view schema.

diesel::table! {
    /// `pg_catalog.pg_matviews` — view showing information about materialized views.
    /// Each row represents a materialized view in the database, providing details
    /// about its name, owner, storage, and refresh state.
    /// Uses `matviewname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_matviews (matviewname) {
        /// Name of the schema containing the materialized view.
        schemaname -> Nullable<Text>,
        /// Name of the materialized view.
        matviewname -> Nullable<Text>,
        /// Name of the materialized view's owner.
        matviewowner -> Nullable<Text>,
        /// Name of the tablespace containing the materialized view (null for default).
        tablespace -> Nullable<Text>,
        /// `true` if the materialized view has (or recently had) any indexes.
        hasindexes -> Nullable<Bool>,
        /// `true` if the materialized view is currently populated.
        ispopulated -> Nullable<Bool>,
        /// The SQL definition (SELECT statement) of the materialized view.
        definition -> Nullable<Text>,
    }
}
