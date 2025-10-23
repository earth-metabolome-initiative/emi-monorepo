//! Submodule for the `information_schema.domain_udt_usage` view schema.

diesel::table! {
    /// `information_schema.domain_udt_usage` — view containing one row for each domain
    /// that is based on a user-defined type. This tracks which domains depend on specific UDTs.
    information_schema.domain_udt_usage (udt_catalog, udt_schema, udt_name, domain_catalog, domain_schema, domain_name) {
        /// Catalog (database) containing the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Catalog (database) containing the domain.
        domain_catalog -> Nullable<Text>,
        /// Schema containing the domain.
        domain_schema -> Nullable<Text>,
        /// Name of the domain.
        domain_name -> Nullable<Text>,
    }
}
