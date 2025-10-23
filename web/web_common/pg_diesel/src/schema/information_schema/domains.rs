//! Submodule for the `information_schema.domains` view schema.

diesel::table! {
    /// `information_schema.domains` — view containing one row for each domain
    /// in the current database that the current user has access to.
    information_schema.domains (domain_catalog, domain_schema, domain_name) {
        /// Catalog (database) containing the domain.
        domain_catalog -> Nullable<Text>,
        /// Schema containing the domain.
        domain_schema -> Nullable<Text>,
        /// Name of the domain.
        domain_name -> Nullable<Text>,
        /// Data type of the domain.
        data_type -> Nullable<Text>,
        /// Maximum length for character domains.
        character_maximum_length -> Nullable<Integer>,
        /// Maximum length in bytes for character domains.
        character_octet_length -> Nullable<Integer>,
        /// Catalog of the character set for character domains.
        character_set_catalog -> Nullable<Text>,
        /// Schema of the character set for character domains.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set for character domains.
        character_set_name -> Nullable<Text>,
        /// Catalog of the collation for character domains.
        collation_catalog -> Nullable<Text>,
        /// Schema of the collation for character domains.
        collation_schema -> Nullable<Text>,
        /// Name of the collation for character domains.
        collation_name -> Nullable<Text>,
        /// Precision for numeric domains.
        numeric_precision -> Nullable<Integer>,
        /// Radix for numeric domains.
        numeric_precision_radix -> Nullable<Integer>,
        /// Scale for numeric domains.
        numeric_scale -> Nullable<Integer>,
        /// Precision for date/time domains.
        datetime_precision -> Nullable<Integer>,
        /// Type of interval domains.
        interval_type -> Nullable<Text>,
        /// Precision of interval domains.
        interval_precision -> Nullable<Integer>,
        /// Default value for the domain.
        domain_default -> Nullable<Text>,
        /// Catalog of the underlying user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema of the underlying user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the underlying user-defined type.
        udt_name -> Nullable<Text>,
        /// Catalog of the scope of a reference domain.
        scope_catalog -> Nullable<Text>,
        /// Schema of the scope of a reference domain.
        scope_schema -> Nullable<Text>,
        /// Name of the scope of a reference domain.
        scope_name -> Nullable<Text>,
        /// Maximum cardinality for array domains.
        maximum_cardinality -> Nullable<Integer>,
        /// DTD identifier for the domain.
        dtd_identifier -> Nullable<Text>,
    }
}
