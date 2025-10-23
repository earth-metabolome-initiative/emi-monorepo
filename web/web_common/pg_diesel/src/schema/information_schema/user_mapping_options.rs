//! `user_mapping_options` view from `information_schema`.

diesel::table! {
    /// `information_schema.user_mapping_options` — view containing one row for each
    /// option of a user mapping accessible to the current user. Used with foreign
    /// data wrappers to configure connection and authentication options.
    information_schema.user_mapping_options (authorization_identifier, foreign_server_catalog, foreign_server_name, option_name) {
        /// Name of the user who owns the user mapping.
        authorization_identifier -> Nullable<Text>,
        /// Name of the database (catalog) containing the foreign server.
        foreign_server_catalog -> Nullable<Text>,
        /// Name of the foreign server.
        foreign_server_name -> Nullable<Text>,
        /// Name of the option.
        option_name -> Nullable<Text>,
        /// Value of the option.
        option_value -> Nullable<Text>,
    }
}
