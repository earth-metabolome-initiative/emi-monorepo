//! Submodule for the `information_schema.collations` view schema.

diesel::table! {
    /// `information_schema.collations` — view containing one row for each collation
    /// available in the current database. Provides information about text collation
    /// rules used for sorting and comparison operations.
    information_schema.collations (collation_catalog, collation_schema, collation_name) {
        /// Catalog (database) containing the collation.
        collation_catalog -> Nullable<Text>,
        /// Schema containing the collation.
        collation_schema -> Nullable<Text>,
        /// Name of the collation.
        collation_name -> Nullable<Text>,
        /// Padding attribute for the collation; typically "PAD SPACE" or "NO PAD".
        pad_attribute -> Nullable<Text>,
    }
}
