//! Submodule for the `information_schema.information_schema_catalog_name` view
//! schema.

diesel::table! {
    /// `information_schema.information_schema_catalog_name` — view containing the name
    /// of the catalog in which the information schema is located.
    information_schema.information_schema_catalog_name (catalog_name) {
        /// Name of the catalog containing the information schema.
        catalog_name -> Nullable<Text>,
    }
}
