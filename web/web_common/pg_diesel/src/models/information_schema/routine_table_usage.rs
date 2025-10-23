//! Model struct for the `information_schema.routine_table_usage` view.
//!
//! This view contains one row for each table used by a routine in the current
//! database.

use diesel::prelude::*;

/// Model struct for `information_schema.routine_table_usage`.
///
/// This view contains one row for each table used by a routine in the current
/// database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::routine_table_usage::routine_table_usage)]
pub struct RoutineTableUsage {
    /// Catalog (database) containing the specific routine.
    pub specific_catalog: Option<String>,
    /// Schema containing the specific routine.
    pub specific_schema: Option<String>,
    /// Specific name of the routine.
    pub specific_name: Option<String>,
    /// Catalog (database) containing the routine.
    pub routine_catalog: Option<String>,
    /// Schema containing the routine.
    pub routine_schema: Option<String>,
    /// Name of the routine.
    pub routine_name: Option<String>,
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
}
