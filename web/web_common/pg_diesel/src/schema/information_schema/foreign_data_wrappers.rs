//! Submodule for the `information_schema.foreign_data_wrappers` view schema.

diesel::table! {
    /// `information_schema.foreign_data_wrappers` — view containing one row for each
    /// foreign data wrapper in the current database.
    information_schema.foreign_data_wrappers (foreign_data_wrapper_catalog, foreign_data_wrapper_name) {
        /// Catalog (database) containing the foreign data wrapper.
        foreign_data_wrapper_catalog -> Nullable<Text>,
        /// Name of the foreign data wrapper.
        foreign_data_wrapper_name -> Nullable<Text>,
        /// Authorization identifier (owner) of the foreign data wrapper.
        authorization_identifier -> Nullable<Text>,
        /// Name of the library that implements the foreign data wrapper.
        library_name -> Nullable<Text>,
        /// Language used to implement the foreign data wrapper.
        foreign_data_wrapper_language -> Nullable<Text>,
    }
}
