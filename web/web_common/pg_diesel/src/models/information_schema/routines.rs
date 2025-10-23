//! Model struct for the `information_schema.routines` view.
//!
//! This view contains one row for each function and procedure in the current
//! database.

use diesel::prelude::*;

/// Model struct for `information_schema.routines`.
///
/// This view contains one row for each function and procedure in the current
/// database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::routines::routines)]
pub struct Routines {
    /// Catalog (database) containing the specific routine.
    pub specific_catalog: Option<String>,
    /// Schema containing the specific routine.
    pub specific_schema: Option<String>,
    /// Specific name of the routine.
    pub specific_name: Option<String>,
    /// Catalog (database) containing the routine.
    pub routine_catalog: Option<String>,
    /// Schema containing the routine.
    pub routine_schema: Option<String>,
    /// Name of the routine.
    pub routine_name: Option<String>,
    /// Type of the routine (FUNCTION or PROCEDURE).
    pub routine_type: Option<String>,
    /// Catalog of the module containing the routine.
    pub module_catalog: Option<String>,
    /// Schema of the module containing the routine.
    pub module_schema: Option<String>,
    /// Name of the module containing the routine.
    pub module_name: Option<String>,
    /// Catalog of the user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema of the user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    pub udt_name: Option<String>,
    /// Data type of the return value.
    pub data_type: Option<String>,
    /// Maximum length for character return types.
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes for character return types.
    pub character_octet_length: Option<i32>,
    /// Catalog of the character set for character return types.
    pub character_set_catalog: Option<String>,
    /// Schema of the character set for character return types.
    pub character_set_schema: Option<String>,
    /// Name of the character set for character return types.
    pub character_set_name: Option<String>,
    /// Catalog of the collation for character return types.
    pub collation_catalog: Option<String>,
    /// Schema of the collation for character return types.
    pub collation_schema: Option<String>,
    /// Name of the collation for character return types.
    pub collation_name: Option<String>,
    /// Precision for numeric return types.
    pub numeric_precision: Option<i32>,
    /// Radix for numeric return types.
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric return types.
    pub numeric_scale: Option<i32>,
    /// Precision for date/time return types.
    pub datetime_precision: Option<i32>,
    /// Type of interval return types.
    pub interval_type: Option<String>,
    /// Precision of interval return types.
    pub interval_precision: Option<i32>,
    /// Catalog of the type user-defined type.
    pub type_udt_catalog: Option<String>,
    /// Schema of the type user-defined type.
    pub type_udt_schema: Option<String>,
    /// Name of the type user-defined type.
    pub type_udt_name: Option<String>,
    /// Catalog of the scope of a reference return type.
    pub scope_catalog: Option<String>,
    /// Schema of the scope of a reference return type.
    pub scope_schema: Option<String>,
    /// Name of the scope of a reference return type.
    pub scope_name: Option<String>,
    /// Maximum cardinality for array return types.
    pub maximum_cardinality: Option<i32>,
    /// DTD identifier for the return type.
    pub dtd_identifier: Option<String>,
    /// How the routine body is implemented.
    pub routine_body: Option<String>,
    /// Definition of the routine.
    pub routine_definition: Option<String>,
    /// External name of the routine.
    pub external_name: Option<String>,
    /// External language of the routine.
    pub external_language: Option<String>,
    /// Parameter style of the routine.
    pub parameter_style: Option<String>,
    /// Whether the routine is deterministic.
    pub is_deterministic: Option<String>,
    /// SQL data access rights of the routine.
    pub sql_data_access: Option<String>,
    /// Whether the routine is called on null input.
    pub is_null_call: Option<String>,
    /// SQL path of the routine.
    pub sql_path: Option<String>,
    /// Whether the routine is a schema-level routine.
    pub schema_level_routine: Option<String>,
    /// Maximum number of dynamic result sets.
    pub max_dynamic_result_sets: Option<i32>,
    /// Whether the routine is a user-defined cast.
    pub is_user_defined_cast: Option<String>,
    /// Whether the routine is implicitly invocable.
    pub is_implicitly_invocable: Option<String>,
    /// Security type of the routine.
    pub security_type: Option<String>,
    /// Catalog of the SQL-specific routine.
    pub to_sql_specific_catalog: Option<String>,
    /// Schema of the SQL-specific routine.
    pub to_sql_specific_schema: Option<String>,
    /// Name of the SQL-specific routine.
    pub to_sql_specific_name: Option<String>,
    /// Whether the routine uses locators.
    pub as_locator: Option<String>,
    /// When the routine was created.
    pub created: Option<std::time::SystemTime>,
    /// When the routine was last altered.
    pub last_altered: Option<std::time::SystemTime>,
    /// New savepoint level.
    pub new_savepoint_level: Option<String>,
    /// Whether the routine is UDT dependent.
    pub is_udt_dependent: Option<String>,
    /// Result cast from data type.
    pub result_cast_from_data_type: Option<String>,
    /// Result cast as locator.
    pub result_cast_as_locator: Option<String>,
    /// Result cast character maximum length.
    pub result_cast_char_max_length: Option<i32>,
    /// Result cast character octet length.
    pub result_cast_char_octet_length: Option<i32>,
    /// Result cast character set catalog.
    pub result_cast_char_set_catalog: Option<String>,
    /// Result cast character set schema.
    pub result_cast_char_set_schema: Option<String>,
    /// Result cast character set name.
    pub result_cast_char_set_name: Option<String>,
    /// Result cast collation catalog.
    pub result_cast_collation_catalog: Option<String>,
    /// Result cast collation schema.
    pub result_cast_collation_schema: Option<String>,
    /// Result cast collation name.
    pub result_cast_collation_name: Option<String>,
    /// Result cast numeric precision.
    pub result_cast_numeric_precision: Option<i32>,
    /// Result cast numeric precision radix.
    pub result_cast_numeric_precision_radix: Option<i32>,
    /// Result cast numeric scale.
    pub result_cast_numeric_scale: Option<i32>,
    /// Result cast datetime precision.
    pub result_cast_datetime_precision: Option<i32>,
    /// Result cast interval type.
    pub result_cast_interval_type: Option<String>,
    /// Result cast interval precision.
    pub result_cast_interval_precision: Option<i32>,
    /// Result cast type UDT catalog.
    pub result_cast_type_udt_catalog: Option<String>,
    /// Result cast type UDT schema.
    pub result_cast_type_udt_schema: Option<String>,
    /// Result cast type UDT name.
    pub result_cast_type_udt_name: Option<String>,
    /// Result cast scope catalog.
    pub result_cast_scope_catalog: Option<String>,
    /// Result cast scope schema.
    pub result_cast_scope_schema: Option<String>,
    /// Result cast scope name.
    pub result_cast_scope_name: Option<String>,
    /// Result cast maximum cardinality.
    pub result_cast_maximum_cardinality: Option<i32>,
    /// Result cast DTD identifier.
    pub result_cast_dtd_identifier: Option<String>,
}
