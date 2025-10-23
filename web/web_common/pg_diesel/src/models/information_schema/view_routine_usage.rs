//! Model struct for the `information_schema.view_routine_usage` view.
//!
//! This view contains one row for each function or procedure that is referenced
//! in the query expression of a view, showing the relationship between views
//! and the underlying routines.

use diesel::prelude::*;

/// Represents a row from the `information_schema.view_routine_usage` view.
/// Contains one row for each function or procedure that is referenced in the
/// query expression of a view, showing the relationship between views and
/// the underlying routines.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::view_routine_usage::view_routine_usage)]
pub struct ViewRoutineUsage {
    /// Name of the database (catalog) containing the view.
    pub table_catalog: Option<String>,
    /// Name of the schema containing the view.
    pub table_schema: Option<String>,
    /// Name of the view.
    pub table_name: Option<String>,
    /// Name of the database (catalog) containing the referenced routine.
    pub specific_catalog: Option<String>,
    /// Name of the schema containing the referenced routine.
    pub specific_schema: Option<String>,
    /// Name of the referenced routine.
    pub specific_name: Option<String>,
}
