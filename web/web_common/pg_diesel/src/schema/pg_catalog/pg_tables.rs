//! Submodule for the `pg_catalog.pg_tables` view schema.

diesel::table! {
    /// `pg_catalog.pg_tables` — view providing access to useful information about each table.
    /// Uses `schemaname` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_tables (schemaname) {
        /// Name of the schema containing the table.
        schemaname -> Nullable<Text>,
        /// Name of the table.
        tablename -> Nullable<Text>,
        /// Name of the table's owner.
        tableowner -> Nullable<Text>,
        /// Name of the tablespace containing the table (null if using the database default).
        tablespace -> Nullable<Text>,
        /// True if table has (or recently had) any indexes.
        hasindexes -> Nullable<Bool>,
        /// True if table has (or once had) rules.
        hasrules -> Nullable<Bool>,
        /// True if table has (or once had) triggers.
        hastriggers -> Nullable<Bool>,
        /// True if row-level security is enabled on the table.
        rowsecurity -> Nullable<Bool>,
    }
}
