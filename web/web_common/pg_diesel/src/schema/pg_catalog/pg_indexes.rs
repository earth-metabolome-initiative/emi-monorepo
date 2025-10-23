//! Submodule for the `pg_indexes` view schema.

diesel::table! {
    /// `pg_indexes` — view containing one row per index in the database.
    /// Provides metadata about indexes, including their schema, table, and definition.
    pg_catalog.pg_indexes (schemaname, tablename, indexname) {
        /// Name of the schema containing the table with the index.
        schemaname -> Text,
        /// Name of the table the index is defined on.
        tablename -> Text,
        /// Name of the index.
        indexname -> Text,
        /// Name of the tablespace the index resides in; `NULL` if using default tablespace.
        tablespace -> Nullable<Text>,
        /// SQL definition of the index (e.g., "CREATE INDEX ...").
        indexdef -> Text,
    }
}
