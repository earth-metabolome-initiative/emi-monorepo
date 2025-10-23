//! Submodule for the `information_schema.column_column_usage` view schema.

diesel::table! {
    /// `information_schema.column_column_usage` — view containing one row for each column
    /// dependency relationship. This tracks cases where one column's definition depends
    /// on another column, such as in computed columns or column constraints.
    information_schema.column_column_usage (table_catalog, table_schema, table_name, column_name, dependent_column) {
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Name of the column that is being depended upon.
        column_name -> Nullable<Text>,
        /// Name of the column that depends on the other column.
        dependent_column -> Nullable<Text>,
    }
}
