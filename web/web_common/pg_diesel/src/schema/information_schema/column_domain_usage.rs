//! Submodule for the `information_schema.column_domain_usage` view schema.

diesel::table! {
    /// `information_schema.column_domain_usage` — view containing one row for each column
    /// that is based on a domain. This tracks which table columns use specific domains
    /// as their data type, providing essential information for domain dependency analysis.
    information_schema.column_domain_usage (domain_catalog, domain_schema, domain_name, table_catalog, table_schema, table_name, column_name) {
        /// Catalog (database) containing the domain.
        domain_catalog -> Nullable<Text>,
        /// Schema containing the domain.
        domain_schema -> Nullable<Text>,
        /// Name of the domain.
        domain_name -> Nullable<Text>,
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Name of the column that uses the domain.
        column_name -> Nullable<Text>,
    }
}
