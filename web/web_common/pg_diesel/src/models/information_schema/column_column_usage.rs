//! Struct representing a row in the `information_schema.column_column_usage`
//! table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.column_column_usage` table.
///
/// The `column_column_usage` view contains one row for each column dependency
/// relationship. This tracks cases where one column's definition depends on
/// another column, such as in computed columns or column constraints.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::column_column_usage::column_column_usage)]
pub struct ColumnColumnUsage {
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Name of the column that is being depended upon.
    pub column_name: Option<String>,
    /// Name of the column that depends on the other column.
    pub dependent_column: Option<String>,
}
