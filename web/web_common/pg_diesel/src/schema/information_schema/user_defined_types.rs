//! `user_defined_types` view from `information_schema`.

diesel::table! {
    /// `information_schema.user_defined_types` — view containing one row for each
    /// user-defined type in the current database. Provides comprehensive metadata
    /// about user-defined types including categorization, character sets, numeric
    /// properties, and data type information.
    information_schema.user_defined_types (user_defined_type_catalog, user_defined_type_schema, user_defined_type_name) {
        /// Name of the database (catalog) containing the user-defined type.
        user_defined_type_catalog -> Nullable<Text>,
        /// Name of the schema containing the user-defined type.
        user_defined_type_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        user_defined_type_name -> Nullable<Text>,
        /// Category of the user-defined type.
        user_defined_type_category -> Nullable<Text>,
        /// "YES" if the type is instantiable, "NO" otherwise.
        is_instantiable -> Nullable<Text>,
        /// "YES" if the type is final, "NO" otherwise.
        is_final -> Nullable<Text>,
        /// Form of ordering for the type.
        ordering_form -> Nullable<Text>,
        /// Category of ordering for the type.
        ordering_category -> Nullable<Text>,
        /// Catalog containing the ordering routine.
        ordering_routine_catalog -> Nullable<Text>,
        /// Schema containing the ordering routine.
        ordering_routine_schema -> Nullable<Text>,
        /// Name of the ordering routine.
        ordering_routine_name -> Nullable<Text>,
        /// Reference type information.
        reference_type -> Nullable<Text>,
        /// Data type of the user-defined type.
        data_type -> Nullable<Text>,
        /// Maximum length for character types.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum octet length for character types.
        character_octet_length -> Nullable<Integer>,
        /// Catalog containing the character set.
        character_set_catalog -> Nullable<Text>,
        /// Schema containing the character set.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set.
        character_set_name -> Nullable<Text>,
        /// Catalog containing the collation.
        collation_catalog -> Nullable<Text>,
        /// Schema containing the collation.
        collation_schema -> Nullable<Text>,
        /// Name of the collation.
        collation_name -> Nullable<Text>,
        /// Precision for numeric types.
        numeric_precision -> Nullable<Integer>,
        /// Radix for numeric precision.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric types.
        numeric_scale -> Nullable<Integer>,
        /// Precision for datetime types.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval for interval types.
        interval_type -> Nullable<Text>,
        /// Precision for interval types.
        interval_precision -> Nullable<Integer>,
        /// Source data type descriptor identifier.
        source_dtd_identifier -> Nullable<Text>,
        /// Reference data type descriptor identifier.
        ref_dtd_identifier -> Nullable<Text>,
    }
}
