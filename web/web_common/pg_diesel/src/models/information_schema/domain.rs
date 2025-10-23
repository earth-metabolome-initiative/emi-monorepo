//! Struct representing a row in the `information_schema.domains` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.domains` table.
///
/// The `domains` view contains one row for each domain in the current database
/// that the current user has access to.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::domains::domains)]
pub struct Domain {
    /// Catalog (database) containing the domain.
    pub domain_catalog: Option<String>,
    /// Schema containing the domain.
    pub domain_schema: Option<String>,
    /// Name of the domain.
    pub domain_name: Option<String>,
    /// Data type of the domain.
    pub data_type: Option<String>,
    /// Maximum length for character domains.
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes for character domains.
    pub character_octet_length: Option<i32>,
    /// Catalog of the character set for character domains.
    pub character_set_catalog: Option<String>,
    /// Schema of the character set for character domains.
    pub character_set_schema: Option<String>,
    /// Name of the character set for character domains.
    pub character_set_name: Option<String>,
    /// Catalog of the collation for character domains.
    pub collation_catalog: Option<String>,
    /// Schema of the collation for character domains.
    pub collation_schema: Option<String>,
    /// Name of the collation for character domains.
    pub collation_name: Option<String>,
    /// Precision for numeric domains.
    pub numeric_precision: Option<i32>,
    /// Radix for numeric domains.
    pub numeric_precision_radix: Option<i32>,
    /// Scale for numeric domains.
    pub numeric_scale: Option<i32>,
    /// Precision for date/time domains.
    pub datetime_precision: Option<i32>,
    /// Type of interval domains.
    pub interval_type: Option<String>,
    /// Precision of interval domains.
    pub interval_precision: Option<i32>,
    /// Default value for the domain.
    pub domain_default: Option<String>,
    /// Catalog of the underlying user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema of the underlying user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the underlying user-defined type.
    pub udt_name: Option<String>,
    /// Catalog of the scope of a reference domain.
    pub scope_catalog: Option<String>,
    /// Schema of the scope of a reference domain.
    pub scope_schema: Option<String>,
    /// Name of the scope of a reference domain.
    pub scope_name: Option<String>,
    /// Maximum cardinality for array domains.
    pub maximum_cardinality: Option<i32>,
    /// DTD identifier for the domain.
    pub dtd_identifier: Option<String>,
}
