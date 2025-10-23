//! Model struct for the `information_schema.view_table_usage` view.
//!
//! This view contains one row for each table that is referenced in the query
//! expression of a view, showing the relationship between views and the
//! underlying tables.

use diesel::prelude::*;

/// Represents a row from the `information_schema.view_table_usage` view.
/// Contains one row for each table that is referenced in the query expression
/// of a view, showing the relationship between views and the underlying tables.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::view_table_usage::view_table_usage)]
pub struct ViewTableUsage {
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
}
