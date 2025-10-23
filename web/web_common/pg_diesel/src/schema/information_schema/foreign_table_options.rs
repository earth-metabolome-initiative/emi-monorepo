//! Submodule for the `information_schema.foreign_table_options` view schema.

diesel::table! {
    /// `information_schema.foreign_table_options` — view containing one row for each
    /// option of a foreign table in the current database.
    information_schema.foreign_table_options (foreign_table_catalog, foreign_table_schema, foreign_table_name, option_name) {
        /// Catalog (database) containing the foreign table.
        foreign_table_catalog -> Nullable<Text>,
        /// Schema containing the foreign table.
        foreign_table_schema -> Nullable<Text>,
        /// Name of the foreign table.
        foreign_table_name -> Nullable<Text>,
        /// Name of the option.
        option_name -> Nullable<Text>,
        /// Value of the option.
        option_value -> Nullable<Text>,
    }
}
