//! Struct representing a row in the `information_schema.attributes` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.attributes` table.
///
/// The `attributes` view contains one row for each attribute of composite types
/// in the current database that the current user has access to. This includes
/// attributes of user-defined types, providing detailed metadata about their
/// structure, data types, constraints, and other properties.
///
/// This view is essential for understanding the composition and schema of
/// custom composite types, including their attribute names, types, default
/// values, nullability, and various type-specific metadata.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::attributes::attributes)]
pub struct Attribute {
    /// Name of the database containing the user-defined type (always the
    /// current database). For PostgreSQL, this is typically the name of the
    /// current database.
    pub udt_catalog: Option<String>,
    /// Name of the schema containing the user-defined type.
    /// This indicates the schema where the composite type is defined.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    /// This is the name of the composite type that contains this attribute.
    pub udt_name: Option<String>,
    /// Name of the attribute.
    /// This is the name of the specific attribute within the composite type.
    pub attribute_name: Option<String>,
    /// Position of the attribute within the user-defined type (1-based).
    /// Attributes are ordered within their containing type, starting from 1.
    pub ordinal_position: Option<i32>,
    /// Default expression for the attribute, if any; otherwise `NULL`.
    /// This contains the default value or expression that will be used if no
    /// value is provided.
    pub attribute_default: Option<String>,
    /// "YES" if the attribute allows NULL values, "NO" otherwise.
    /// This indicates whether the attribute can contain NULL values.
    pub __is_nullable: Option<String>,
    /// Data type of the attribute (e.g., "integer", "text", "timestamp").
    /// This is the SQL data type name for the attribute.
    pub data_type: Option<String>,
    /// Maximum length for character attributes (number of characters); `NULL`
    /// if not applicable. For character and text types, this specifies the
    /// maximum number of characters allowed.
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes for character attributes; `NULL` if not
    /// applicable. For character types, this specifies the maximum storage
    /// size in bytes.
    pub character_octet_length: Option<i32>,
    /// Catalog of the character set for character attributes; `NULL` if not
    /// applicable. Identifies the catalog containing the character set
    /// definition.
    pub character_set_catalog: Option<String>,
    /// Schema of the character set for character attributes; `NULL` if not
    /// applicable. Identifies the schema containing the character set
    /// definition.
    pub character_set_schema: Option<String>,
    /// Name of the character set for character attributes; `NULL` if not
    /// applicable. The name of the character set used for character data
    /// encoding.
    pub character_set_name: Option<String>,
    /// Catalog of the collation for character attributes; `NULL` if not
    /// applicable. Identifies the catalog containing the collation
    /// definition.
    pub collation_catalog: Option<String>,
    /// Schema of the collation for character attributes; `NULL` if not
    /// applicable. Identifies the schema containing the collation
    /// definition.
    pub collation_schema: Option<String>,
    /// Name of the collation for character attributes; `NULL` if not
    /// applicable. The name of the collation used for character data
    /// sorting and comparison.
    pub collation_name: Option<String>,
    /// Precision for numeric attributes; `NULL` if not applicable.
    /// For numeric types, this is the total number of significant digits.
    pub numeric_precision: Option<i32>,
    /// Radix (base) for numeric attributes; typically 2 or 10; `NULL` if not
    /// applicable. The base used for representing the precision (binary or
    /// decimal).
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric attributes; `NULL` if not applicable.
    /// For numeric types, this is the number of digits after the decimal point.
    pub numeric_scale: Option<i32>,
    /// Precision for date/time attributes (number of fractional seconds
    /// digits); `NULL` if not applicable. For timestamp and time types,
    /// this specifies the precision of fractional seconds.
    pub datetime_precision: Option<i32>,
    /// Type of interval attributes (e.g., "YEAR TO MONTH"); `NULL` if not
    /// applicable. For interval types, this describes the specific interval
    /// fields included.
    pub interval_type: Option<String>,
    /// Precision of interval attributes; `NULL` if not applicable.
    /// For interval types, this specifies the precision of the interval value.
    pub interval_precision: Option<i32>,
    /// Catalog of the underlying user-defined type (UDT) for this attribute;
    /// `NULL` if not applicable. If the attribute itself is of a
    /// user-defined type, this identifies the catalog.
    pub attribute_udt_catalog: Option<String>,
    /// Schema of the UDT for this attribute; `NULL` if not applicable.
    /// If the attribute itself is of a user-defined type, this identifies the
    /// schema.
    pub attribute_udt_schema: Option<String>,
    /// Name of the UDT for this attribute; `NULL` if not applicable.
    /// If the attribute itself is of a user-defined type, this is the type
    /// name.
    pub attribute_udt_name: Option<String>,
    /// Catalog of the scope of a reference attribute (foreign key); `NULL` if
    /// not applicable. For reference types, this identifies the catalog of
    /// the referenced scope.
    pub scope_catalog: Option<String>,
    /// Schema of the scope of a reference attribute; `NULL` if not applicable.
    /// For reference types, this identifies the schema of the referenced scope.
    pub scope_schema: Option<String>,
    /// Name of the scope of a reference attribute; `NULL` if not applicable.
    /// For reference types, this identifies the name of the referenced scope.
    pub scope_name: Option<String>,
    /// Maximum cardinality for array attributes; `NULL` if not applicable.
    /// For array types, this specifies the maximum number of elements allowed.
    pub maximum_cardinality: Option<i32>,
    /// DTD identifier for the attribute; `NULL` if not applicable.
    /// A unique identifier used in the data type descriptor for this attribute.
    pub dtd_identifier: Option<String>,
    /// "YES" if the attribute is derived from a reference type; `NULL` if not
    /// applicable. Indicates whether this attribute is inherited from a
    /// reference type.
    pub is_derived_reference_attribute: Option<String>,
}
