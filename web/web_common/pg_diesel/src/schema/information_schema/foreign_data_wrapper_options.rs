//! Submodule for the `information_schema.foreign_data_wrapper_options` view
//! schema.

diesel::table! {
    /// `information_schema.foreign_data_wrapper_options` — view containing one row for each
    /// option of a foreign data wrapper in the current database.
    information_schema.foreign_data_wrapper_options (foreign_data_wrapper_catalog, foreign_data_wrapper_name, option_name) {
        /// Catalog (database) containing the foreign data wrapper.
        foreign_data_wrapper_catalog -> Nullable<Text>,
        /// Name of the foreign data wrapper.
        foreign_data_wrapper_name -> Nullable<Text>,
        /// Name of the option.
        option_name -> Nullable<Text>,
        /// Value of the option.
        option_value -> Nullable<Text>,
    }
}
