//! Submodule for the `information_schema.element_types` view schema.

diesel::table! {
    /// `information_schema.element_types` — view containing one row for each data type
    /// descriptor that is used by elements in SQL data structures in the current database.
    information_schema.element_types (object_catalog, object_schema, object_name, object_type, collection_type_identifier) {
        /// Catalog (database) containing the object.
        object_catalog -> Nullable<Text>,
        /// Schema containing the object.
        object_schema -> Nullable<Text>,
        /// Name of the object.
        object_name -> Nullable<Text>,
        /// Type of the object.
        object_type -> Nullable<Text>,
        /// Collection type identifier.
        collection_type_identifier -> Nullable<Text>,
        /// Data type of the element.
        data_type -> Nullable<Text>,
        /// Maximum length for character elements.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character elements.
        character_octet_length -> Nullable<Integer>,
        /// Catalog of the character set for character elements.
        character_set_catalog -> Nullable<Text>,
        /// Schema of the character set for character elements.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set for character elements.
        character_set_name -> Nullable<Text>,
        /// Catalog of the collation for character elements.
        collation_catalog -> Nullable<Text>,
        /// Schema of the collation for character elements.
        collation_schema -> Nullable<Text>,
        /// Name of the collation for character elements.
        collation_name -> Nullable<Text>,
        /// Precision for numeric elements.
        numeric_precision -> Nullable<Integer>,
        /// Radix for numeric elements.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric elements.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time elements.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval elements.
        interval_type -> Nullable<Text>,
        /// Precision of interval elements.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the underlying user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema of the underlying user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the underlying user-defined type.
        udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference element.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference element.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference element.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array elements.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the element.
        dtd_identifier -> Nullable<Text>,
    }
}
