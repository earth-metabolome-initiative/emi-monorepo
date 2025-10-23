//! Submodule for the `information_schema.column_options` view schema.

diesel::table! {
    /// `information_schema.column_options` — view containing one row for each option
    /// set on a column. This provides access to column-specific configuration options.
    information_schema.column_options (table_catalog, table_schema, table_name, column_name, option_name) {
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Name of the column.
        column_name -> Nullable<Text>,
        /// Name of the option.
        option_name -> Nullable<Text>,
        /// Value of the option.
        option_value -> Nullable<Text>,
    }
}
