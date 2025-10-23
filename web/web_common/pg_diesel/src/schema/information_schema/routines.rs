//! Submodule for the `information_schema.routines` view schema.

diesel::table! {
    /// `information_schema.routines` — view containing one row for each
    /// function and procedure in the current database.
    information_schema.routines (specific_catalog, specific_schema, specific_name) {
        /// Catalog (database) containing the specific routine.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the specific routine.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine.
        specific_name -> Nullable<Text>,
        /// Catalog (database) containing the routine.
        routine_catalog -> Nullable<Text>,
        /// Schema containing the routine.
        routine_schema -> Nullable<Text>,
        /// Name of the routine.
        routine_name -> Nullable<Text>,
        /// Type of the routine (FUNCTION or PROCEDURE).
        routine_type -> Nullable<Text>,
        /// Catalog of the module containing the routine.
        module_catalog -> Nullable<Text>,
        /// Schema of the module containing the routine.
        module_schema -> Nullable<Text>,
        /// Name of the module containing the routine.
        module_name -> Nullable<Text>,
        /// Catalog of the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema of the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Data type of the return value.
        data_type -> Nullable<Text>,
        /// Maximum length for character return types.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character return types.
        character_octet_length -> Nullable<Integer>,
        /// Catalog of the character set for character return types.
        character_set_catalog -> Nullable<Text>,
        /// Schema of the character set for character return types.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set for character return types.
        character_set_name -> Nullable<Text>,
        /// Catalog of the collation for character return types.
        collation_catalog -> Nullable<Text>,
        /// Schema of the collation for character return types.
        collation_schema -> Nullable<Text>,
        /// Name of the collation for character return types.
        collation_name -> Nullable<Text>,
        /// Precision for numeric return types.
        numeric_precision -> Nullable<Integer>,
        /// Radix for numeric return types.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric return types.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time return types.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval return types.
        interval_type -> Nullable<Text>,
        /// Precision of interval return types.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the type user-defined type.
        type_udt_catalog -> Nullable<Text>,
        /// Schema of the type user-defined type.
        type_udt_schema -> Nullable<Text>,
        /// Name of the type user-defined type.
        type_udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference return type.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference return type.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference return type.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array return types.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the return type.
        dtd_identifier -> Nullable<Text>,
        /// How the routine body is implemented.
        routine_body -> Nullable<Text>,
        /// Definition of the routine.
        routine_definition -> Nullable<Text>,
        /// External name of the routine.
        external_name -> Nullable<Text>,
        /// External language of the routine.
        external_language -> Nullable<Text>,
        /// Parameter style of the routine.
        parameter_style -> Nullable<Text>,
        /// Whether the routine is deterministic.
        is_deterministic -> Nullable<Text>,
        /// SQL data access rights of the routine.
        sql_data_access -> Nullable<Text>,
        /// Whether the routine is called on null input.
        is_null_call -> Nullable<Text>,
        /// SQL path of the routine.
        sql_path -> Nullable<Text>,
        /// Whether the routine is a schema-level routine.
        schema_level_routine -> Nullable<Text>,
        /// Maximum number of dynamic result sets.
        max_dynamic_result_sets -> Nullable<Integer>,
        /// Whether the routine is a user-defined cast.
        is_user_defined_cast -> Nullable<Text>,
        /// Whether the routine is implicitly invocable.
        is_implicitly_invocable -> Nullable<Text>,
        /// Security type of the routine.
        security_type -> Nullable<Text>,
        /// Catalog of the SQL-specific routine.
        to_sql_specific_catalog -> Nullable<Text>,
        /// Schema of the SQL-specific routine.
        to_sql_specific_schema -> Nullable<Text>,
        /// Name of the SQL-specific routine.
        to_sql_specific_name -> Nullable<Text>,
        /// Whether the routine uses locators.
        as_locator -> Nullable<Text>,
        /// When the routine was created.
        created -> Nullable<Timestamp>,
        /// When the routine was last altered.
        last_altered -> Nullable<Timestamp>,
        /// New savepoint level.
        new_savepoint_level -> Nullable<Text>,
        /// Whether the routine is UDT dependent.
        is_udt_dependent -> Nullable<Text>,
        /// Result cast from data type.
        result_cast_from_data_type -> Nullable<Text>,
        /// Result cast as locator.
        result_cast_as_locator -> Nullable<Text>,
        /// Result cast character maximum length.
        result_cast_char_max_length -> Nullable<Integer>,
        /// Result cast character octet length.
        result_cast_char_octet_length -> Nullable<Integer>,
        /// Result cast character set catalog.
        result_cast_char_set_catalog -> Nullable<Text>,
        /// Result cast character set schema.
        result_cast_char_set_schema -> Nullable<Text>,
        /// Result cast character set name.
        result_cast_char_set_name -> Nullable<Text>,
        /// Result cast collation catalog.
        result_cast_collation_catalog -> Nullable<Text>,
        /// Result cast collation schema.
        result_cast_collation_schema -> Nullable<Text>,
        /// Result cast collation name.
        result_cast_collation_name -> Nullable<Text>,
        /// Result cast numeric precision.
        result_cast_numeric_precision -> Nullable<Integer>,
        /// Result cast numeric precision radix.
        result_cast_numeric_precision_radix -> Nullable<Integer>,
        /// Result cast numeric scale.
        result_cast_numeric_scale -> Nullable<Integer>,
        /// Result cast datetime precision.
        result_cast_datetime_precision -> Nullable<Integer>,
        /// Result cast interval type.
        result_cast_interval_type -> Nullable<Text>,
        /// Result cast interval precision.
        result_cast_interval_precision -> Nullable<Integer>,
        /// Result cast type UDT catalog.
        result_cast_type_udt_catalog -> Nullable<Text>,
        /// Result cast type UDT schema.
        result_cast_type_udt_schema -> Nullable<Text>,
        /// Result cast type UDT name.
        result_cast_type_udt_name -> Nullable<Text>,
        /// Result cast scope catalog.
        result_cast_scope_catalog -> Nullable<Text>,
        /// Result cast scope schema.
        result_cast_scope_schema -> Nullable<Text>,
        /// Result cast scope name.
        result_cast_scope_name -> Nullable<Text>,
        /// Result cast maximum cardinality.
        result_cast_maximum_cardinality -> Nullable<Integer>,
        /// Result cast DTD identifier.
        result_cast_dtd_identifier -> Nullable<Text>,
    }
}
