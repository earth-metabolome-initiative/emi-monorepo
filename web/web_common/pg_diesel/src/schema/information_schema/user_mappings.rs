//! `user_mappings` view from `information_schema`.

diesel::table! {
    /// `information_schema.user_mappings` — view containing one row for each
    /// user mapping defined in the current database that is accessible to the
    /// current user. Used with foreign data wrappers for user authentication.
    information_schema.user_mappings (authorization_identifier, foreign_server_catalog, foreign_server_name) {
        /// Name of the user who owns the user mapping.
        authorization_identifier -> Nullable<Text>,
        /// Name of the database (catalog) containing the foreign server.
        foreign_server_catalog -> Nullable<Text>,
        /// Name of the foreign server.
        foreign_server_name -> Nullable<Text>,
    }
}
