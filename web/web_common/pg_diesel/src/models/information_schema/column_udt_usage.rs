//! Struct representing a row in the `information_schema.column_udt_usage`
//! table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.column_udt_usage` table.
///
/// The `column_udt_usage` view contains one row for each column that uses
/// a user-defined type. This tracks which columns depend on specific UDTs.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::column_udt_usage::column_udt_usage)]
pub struct ColumnUdtUsage {
    /// Catalog (database) containing the user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema containing the user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    pub udt_name: Option<String>,
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Name of the column that uses the UDT.
    pub column_name: Option<String>,
}
