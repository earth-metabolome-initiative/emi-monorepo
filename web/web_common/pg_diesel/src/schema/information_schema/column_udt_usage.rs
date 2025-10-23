//! Submodule for the `information_schema.column_udt_usage` view schema.

diesel::table! {
    /// `information_schema.column_udt_usage` — view containing one row for each column
    /// that uses a user-defined type. This tracks which columns depend on specific UDTs.
    information_schema.column_udt_usage (udt_catalog, udt_schema, udt_name, table_catalog, table_schema, table_name, column_name) {
        /// Catalog (database) containing the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Name of the column that uses the UDT.
        column_name -> Nullable<Text>,
    }
}
