//! Parameters model representing the information_schema.parameters view.

use diesel::prelude::*;

/// Model struct for `information_schema.parameters`.
///
/// This view contains one row for each parameter of a function or procedure in
/// the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::parameters::parameters)]
pub struct Parameters {
    /// Catalog (database) containing the function or procedure.
    pub specific_catalog: Option<String>,
    /// Schema containing the function or procedure.
    pub specific_schema: Option<String>,
    /// Name of the function or procedure.
    pub specific_name: Option<String>,
    /// Ordinal position of the parameter within the function signature.
    pub ordinal_position: Option<i32>,
    /// Mode of the parameter (IN, OUT, INOUT).
    pub parameter_mode: Option<String>,
    /// Whether this parameter represents a result.
    pub is_result: Option<String>,
    /// Whether the parameter is used as a locator.
    pub as_locator: Option<String>,
    /// Name of the parameter.
    pub parameter_name: Option<String>,
    /// Data type of the parameter.
    pub data_type: Option<String>,
    /// Maximum length for character parameters.
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes for character parameters.
    pub character_octet_length: Option<i32>,
    /// Catalog of the character set for character parameters.
    pub character_set_catalog: Option<String>,
    /// Schema of the character set for character parameters.
    pub character_set_schema: Option<String>,
    /// Name of the character set for character parameters.
    pub character_set_name: Option<String>,
    /// Catalog of the collation for character parameters.
    pub collation_catalog: Option<String>,
    /// Schema of the collation for character parameters.
    pub collation_schema: Option<String>,
    /// Name of the collation for character parameters.
    pub collation_name: Option<String>,
    /// Precision for numeric parameters.
    pub numeric_precision: Option<i32>,
    /// Radix for numeric parameters.
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric parameters.
    pub numeric_scale: Option<i32>,
    /// Precision for date/time parameters.
    pub datetime_precision: Option<i32>,
    /// Type of interval parameters.
    pub interval_type: Option<String>,
    /// Precision of interval parameters.
    pub interval_precision: Option<i32>,
    /// Catalog of the underlying user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema of the underlying user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the underlying user-defined type.
    pub udt_name: Option<String>,
    /// Catalog of the scope of a reference parameter.
    pub scope_catalog: Option<String>,
    /// Schema of the scope of a reference parameter.
    pub scope_schema: Option<String>,
    /// Name of the scope of a reference parameter.
    pub scope_name: Option<String>,
    /// Maximum cardinality for array parameters.
    pub maximum_cardinality: Option<i32>,
    /// DTD identifier for the parameter.
    pub dtd_identifier: Option<String>,
    /// Default value for the parameter.
    pub parameter_default: Option<String>,
}
