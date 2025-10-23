//! `schemata` view from `information_schema`.

diesel::table! {
    /// `information_schema.schemata` — view containing one row for each schema
    /// in the current database that the current user has access to.
    /// Provides metadata about database schemas including catalog, schema name,
    /// owner, and default character set information.
    information_schema.schemata (catalog_name, schema_name) {
        /// Name of the database (catalog) containing the schema.
        catalog_name -> Text,
        /// Name of the schema.
        schema_name -> Text,
        /// Name of the user who owns the schema.
        schema_owner -> Text,
        /// Name of the default character set for the schema.
        default_character_set_catalog -> Nullable<Text>,
        /// Schema containing the default character set.
        default_character_set_schema -> Nullable<Text>,
        /// Name of the default character set.
        default_character_set_name -> Nullable<Text>,
        /// SQL path for the schema (typically NULL in PostgreSQL).
        sql_path -> Nullable<Text>,
    }
}
