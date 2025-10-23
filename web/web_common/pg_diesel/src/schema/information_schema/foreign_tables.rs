//! Submodule for the `information_schema.foreign_tables` view schema.

diesel::table! {
    /// `information_schema.foreign_tables` — view containing one row for each
    /// foreign table in the current database.
    information_schema.foreign_tables (foreign_table_catalog, foreign_table_schema, foreign_table_name) {
        /// Catalog (database) containing the foreign table.
        foreign_table_catalog -> Nullable<Text>,
        /// Schema containing the foreign table.
        foreign_table_schema -> Nullable<Text>,
        /// Name of the foreign table.
        foreign_table_name -> Nullable<Text>,
        /// Catalog (database) containing the foreign server.
        foreign_server_catalog -> Nullable<Text>,
        /// Name of the foreign server.
        foreign_server_name -> Nullable<Text>,
    }
}
