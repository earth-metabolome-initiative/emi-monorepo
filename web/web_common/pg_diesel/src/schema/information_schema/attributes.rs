//! Submodule for the `information_schema.attributes` view schema.

diesel::table! {
    /// `information_schema.attributes` — view containing one row for each attribute of composite types
    /// in the current database that the current user has access to. Provides detailed metadata about
    /// user-defined type attributes including data types, constraints, defaults, and type information.
    information_schema.attributes (udt_catalog, udt_schema, udt_name, attribute_name) {
        /// Name of the database containing the user-defined type (always the current database).
        udt_catalog -> Nullable<Text>,
        /// Name of the schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Name of the attribute.
        attribute_name -> Nullable<Text>,
        /// Position of the attribute within the user-defined type (1-based).
        ordinal_position -> Nullable<Integer>,
        /// Default expression for the attribute, if any; otherwise `NULL`.
        attribute_default -> Nullable<Text>,
        /// "YES" if the attribute allows NULL values, "NO" otherwise.
        #[sql_name = "is_nullable"]
        __is_nullable -> Nullable<Text>,
        /// Data type of the attribute (e.g., "integer", "text", "timestamp").
        data_type -> Nullable<Text>,
        /// Maximum length for character attributes (number of characters); `NULL` if not applicable.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character attributes; `NULL` if not applicable.
        character_octet_length -> Nullable<Integer>,
        /// Catalog of the character set for character attributes; `NULL` if not applicable.
        character_set_catalog -> Nullable<Text>,
        /// Schema of the character set for character attributes; `NULL` if not applicable.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set for character attributes; `NULL` if not applicable.
        character_set_name -> Nullable<Text>,
        /// Catalog of the collation for character attributes; `NULL` if not applicable.
        collation_catalog -> Nullable<Text>,
        /// Schema of the collation for character attributes; `NULL` if not applicable.
        collation_schema -> Nullable<Text>,
        /// Name of the collation for character attributes; `NULL` if not applicable.
        collation_name -> Nullable<Text>,
        /// Precision for numeric attributes; `NULL` if not applicable.
        numeric_precision -> Nullable<Integer>,
        /// Radix (base) for numeric attributes; typically 2 or 10; `NULL` if not applicable.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric attributes; `NULL` if not applicable.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time attributes (number of fractional seconds digits); `NULL` if not applicable.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval attributes (e.g., "YEAR TO MONTH"); `NULL` if not applicable.
        interval_type -> Nullable<Text>,
        /// Precision of interval attributes; `NULL` if not applicable.
        interval_precision -> Nullable<Integer>,
        /// Catalog of the underlying user-defined type (UDT) for this attribute; `NULL` if not applicable.
        attribute_udt_catalog -> Nullable<Text>,
        /// Schema of the UDT for this attribute; `NULL` if not applicable.
        attribute_udt_schema -> Nullable<Text>,
        /// Name of the UDT for this attribute; `NULL` if not applicable.
        attribute_udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference attribute (foreign key); `NULL` if not applicable.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference attribute; `NULL` if not applicable.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference attribute; `NULL` if not applicable.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array attributes; `NULL` if not applicable.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the attribute; `NULL` if not applicable.
        dtd_identifier -> Nullable<Text>,
        /// "YES" if the attribute is derived from a reference type; `NULL` if not applicable.
        is_derived_reference_attribute -> Nullable<Text>,
    }
}
