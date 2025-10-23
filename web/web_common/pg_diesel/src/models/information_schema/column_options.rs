//! Struct representing a row in the `information_schema.column_options` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.column_options` table.
///
/// The `column_options` view contains one row for each option set on a column.
/// This provides access to column-specific configuration options.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::column_options::column_options)]
pub struct ColumnOptions {
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Name of the column.
    pub column_name: Option<String>,
    /// Name of the option.
    pub option_name: Option<String>,
    /// Value of the option.
    pub option_value: Option<String>,
}
