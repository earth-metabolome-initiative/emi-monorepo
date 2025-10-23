//! Submodule for the `information_schema.foreign_servers` view schema.

diesel::table! {
    /// `information_schema.foreign_servers` — view containing one row for each
    /// foreign server in the current database.
    information_schema.foreign_servers (foreign_server_catalog, foreign_server_name) {
        /// Catalog (database) containing the foreign server.
        foreign_server_catalog -> Nullable<Text>,
        /// Name of the foreign server.
        foreign_server_name -> Nullable<Text>,
        /// Catalog (database) containing the foreign data wrapper.
        foreign_data_wrapper_catalog -> Nullable<Text>,
        /// Name of the foreign data wrapper.
        foreign_data_wrapper_name -> Nullable<Text>,
        /// Type of the foreign server.
        foreign_server_type -> Nullable<Text>,
        /// Version of the foreign server.
        foreign_server_version -> Nullable<Text>,
        /// Authorization identifier (owner) of the foreign server.
        authorization_identifier -> Nullable<Text>,
    }
}
