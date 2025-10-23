//! Submodule for the `information_schema.data_type_privileges` view schema.

diesel::table! {
    /// `information_schema.data_type_privileges` — view containing one row for each
    /// data type privilege. This provides information about data type access permissions.
    information_schema.data_type_privileges (object_catalog, object_schema, object_name, object_type, dtd_identifier) {
        /// Catalog (database) containing the object.
        object_catalog -> Nullable<Text>,
        /// Schema containing the object.
        object_schema -> Nullable<Text>,
        /// Name of the object.
        object_name -> Nullable<Text>,
        /// Type of the object.
        object_type -> Nullable<Text>,
        /// DTD identifier for the data type.
        dtd_identifier -> Nullable<Text>,
    }
}
