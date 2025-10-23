//! Model struct for the `information_schema.element_types` view.
//!
//! This view contains one row for each data type descriptor that is used by
//! elements in SQL data structures in the current database.

use diesel::prelude::*;

/// Model struct for `information_schema.element_types`.
///
/// This view contains one row for each data type descriptor that is used by
/// elements in SQL data structures in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::element_types::element_types)]
pub struct ElementTypes {
    /// Catalog (database) containing the object.
    pub object_catalog: Option<String>,
    /// Schema containing the object.
    pub object_schema: Option<String>,
    /// Name of the object.
    pub object_name: Option<String>,
    /// Type of the object.
    pub object_type: Option<String>,
    /// Collection type identifier.
    pub collection_type_identifier: Option<String>,
    /// Data type of the element.
    pub data_type: Option<String>,
    /// Maximum length for character elements.
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes for character elements.
    pub character_octet_length: Option<i32>,
    /// Catalog of the character set for character elements.
    pub character_set_catalog: Option<String>,
    /// Schema of the character set for character elements.
    pub character_set_schema: Option<String>,
    /// Name of the character set for character elements.
    pub character_set_name: Option<String>,
    /// Catalog of the collation for character elements.
    pub collation_catalog: Option<String>,
    /// Schema of the collation for character elements.
    pub collation_schema: Option<String>,
    /// Name of the collation for character elements.
    pub collation_name: Option<String>,
    /// Precision for numeric elements.
    pub numeric_precision: Option<i32>,
    /// Radix for numeric elements.
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric elements.
    pub numeric_scale: Option<i32>,
    /// Precision for date/time elements.
    pub datetime_precision: Option<i32>,
    /// Type of interval elements.
    pub interval_type: Option<String>,
    /// Precision of interval elements.
    pub interval_precision: Option<i32>,
    /// Catalog of the underlying user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema of the underlying user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the underlying user-defined type.
    pub udt_name: Option<String>,
    /// Catalog of the scope of a reference element.
    pub scope_catalog: Option<String>,
    /// Schema of the scope of a reference element.
    pub scope_schema: Option<String>,
    /// Name of the scope of a reference element.
    pub scope_name: Option<String>,
    /// Maximum cardinality for array elements.
    pub maximum_cardinality: Option<i32>,
    /// DTD identifier for the element.
    pub dtd_identifier: Option<String>,
}
