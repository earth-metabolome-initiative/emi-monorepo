//! Submodule for the `pg_catalog.pg_publication_tables` view schema.

diesel::table! {
    /// `pg_catalog.pg_publication_tables` — view showing tables included in publications.
    /// Each row represents a table that is part of a publication with details about
    /// which columns are published and any row filters.
    /// Uses `pubname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_publication_tables (pubname) {
        /// Name of the publication.
        pubname -> Nullable<Text>,
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table.
        tablename -> Nullable<Text>,
        /// Names of columns included in the publication (null means all columns).
        attnames -> Nullable<Array<Text>>,
        /// Row filter expression (null if no filter).
        rowfilter -> Nullable<Text>,
    }
}
