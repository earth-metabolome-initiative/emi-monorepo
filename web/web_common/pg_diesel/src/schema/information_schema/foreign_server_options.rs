//! Submodule for the `information_schema.foreign_server_options` view schema.

diesel::table! {
    /// `information_schema.foreign_server_options` — view containing one row for each
    /// option of a foreign server in the current database.
    information_schema.foreign_server_options (foreign_server_catalog, foreign_server_name, option_name) {
        /// Catalog (database) containing the foreign server.
        foreign_server_catalog -> Nullable<Text>,
        /// Name of the foreign server.
        foreign_server_name -> Nullable<Text>,
        /// Name of the option.
        option_name -> Nullable<Text>,
        /// Value of the option.
        option_value -> Nullable<Text>,
    }
}
