//! Model struct for the `information_schema.view_column_usage` view.
//!
//! This view contains one row for each column that is referenced in the query
//! expression of a view, showing the relationship between views and the
//! underlying table columns.

use diesel::prelude::*;

/// Represents a row from the `information_schema.view_column_usage` view.
/// Contains one row for each column that is referenced in the query expression
/// of a view, showing the relationship between views and the underlying table
/// columns.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::view_column_usage::view_column_usage)]
pub struct ViewColumnUsage {
    /// Name of the database (catalog) containing the view.
    pub view_catalog: Option<String>,
    /// Name of the schema containing the view.
    pub view_schema: Option<String>,
    /// Name of the view.
    pub view_name: Option<String>,
    /// Name of the database (catalog) containing the referenced table.
    pub table_catalog: Option<String>,
    /// Name of the schema containing the referenced table.
    pub table_schema: Option<String>,
    /// Name of the referenced table.
    pub table_name: Option<String>,
    /// Name of the referenced column.
    pub column_name: Option<String>,
}
