//! Model struct for the `information_schema.user_defined_types` view.
//!
//! This view contains one row for each user-defined type in the current
//! database, providing comprehensive metadata about user-defined types
//! including categorization, character sets, numeric properties, and data type
//! information.

use diesel::prelude::*;

/// Represents a row from the `information_schema.user_defined_types` view.
/// Contains one row for each user-defined type in the current database,
/// providing comprehensive metadata about user-defined types including
/// categorization, character sets, numeric properties, and data type
/// information.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::user_defined_types::user_defined_types)]
pub struct UserDefinedTypes {
    /// Name of the database (catalog) containing the user-defined type.
    pub user_defined_type_catalog: Option<String>,
    /// Name of the schema containing the user-defined type.
    pub user_defined_type_schema: Option<String>,
    /// Name of the user-defined type.
    pub user_defined_type_name: Option<String>,
    /// Category of the user-defined type.
    pub user_defined_type_category: Option<String>,
    /// "YES" if the type is instantiable, "NO" otherwise.
    pub is_instantiable: Option<String>,
    /// "YES" if the type is final, "NO" otherwise.
    pub is_final: Option<String>,
    /// Form of ordering for the type.
    pub ordering_form: Option<String>,
    /// Category of ordering for the type.
    pub ordering_category: Option<String>,
    /// Catalog containing the ordering routine.
    pub ordering_routine_catalog: Option<String>,
    /// Schema containing the ordering routine.
    pub ordering_routine_schema: Option<String>,
    /// Name of the ordering routine.
    pub ordering_routine_name: Option<String>,
    /// Reference type information.
    pub reference_type: Option<String>,
    /// Data type of the user-defined type.
    pub data_type: Option<String>,
    /// Maximum length for character types.
    pub character_maximum_length: Option<i32>,
    /// Maximum octet length for character types.
    pub character_octet_length: Option<i32>,
    /// Catalog containing the character set.
    pub character_set_catalog: Option<String>,
    /// Schema containing the character set.
    pub character_set_schema: Option<String>,
    /// Name of the character set.
    pub character_set_name: Option<String>,
    /// Catalog containing the collation.
    pub collation_catalog: Option<String>,
    /// Schema containing the collation.
    pub collation_schema: Option<String>,
    /// Name of the collation.
    pub collation_name: Option<String>,
    /// Precision for numeric types.
    pub numeric_precision: Option<i32>,
    /// Radix for numeric precision.
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric types.
    pub numeric_scale: Option<i32>,
    /// Precision for datetime types.
    pub datetime_precision: Option<i32>,
    /// Type of interval for interval types.
    pub interval_type: Option<String>,
    /// Precision for interval types.
    pub interval_precision: Option<i32>,
    /// Source data type descriptor identifier.
    pub source_dtd_identifier: Option<String>,
    /// Reference data type descriptor identifier.
    pub ref_dtd_identifier: Option<String>,
}
