//! Submodule for the `information_schema.parameters` view schema.

diesel::table! {
    /// `information_schema.parameters` — view containing one row for each parameter
    /// of a function or procedure in the current database.
    information_schema.parameters (specific_catalog, specific_schema, specific_name, ordinal_position) {
        /// Catalog (database) containing the function or procedure.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the function or procedure.
        specific_schema -> Nullable<Text>,
        /// Name of the function or procedure.
        specific_name -> Nullable<Text>,
        /// Ordinal position of the parameter within the function signature.
        ordinal_position -> Nullable<Integer>,
        /// Mode of the parameter (IN, OUT, INOUT).
        parameter_mode -> Nullable<Text>,
        /// Whether this parameter represents a result.
        is_result -> Nullable<Text>,
        /// Whether the parameter is used as a locator.
        as_locator -> Nullable<Text>,
        /// Name of the parameter.
        parameter_name -> Nullable<Text>,
        /// Data type of the parameter.
        data_type -> Nullable<Text>,
        /// Maximum length for character parameters.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character parameters.
        character_octet_length -> Nullable<Integer>,
        /// Catalog of the character set for character parameters.
        character_set_catalog -> Nullable<Text>,
        /// Schema of the character set for character parameters.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set for character parameters.
        character_set_name -> Nullable<Text>,
        /// Catalog of the collation for character parameters.
        collation_catalog -> Nullable<Text>,
        /// Schema of the collation for character parameters.
        collation_schema -> Nullable<Text>,
        /// Name of the collation for character parameters.
        collation_name -> Nullable<Text>,
        /// Precision for numeric parameters.
        numeric_precision -> Nullable<Integer>,
        /// Radix for numeric parameters.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric parameters.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time parameters.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval parameters.
        interval_type -> Nullable<Text>,
        /// Precision of interval parameters.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the underlying user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema of the underlying user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the underlying user-defined type.
        udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference parameter.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference parameter.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference parameter.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array parameters.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the parameter.
        dtd_identifier -> Nullable<Text>,
        /// Default value for the parameter.
        parameter_default -> Nullable<Text>,
    }
}
