//! Model struct for the `information_schema.routine_sequence_usage` view.
//!
//! This view contains one row for each sequence used by a routine in the
//! current database.

use diesel::prelude::*;

/// Model struct for `information_schema.routine_sequence_usage`.
///
/// This view contains one row for each sequence used by a routine in the
/// current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::routine_sequence_usage::routine_sequence_usage)]
pub struct RoutineSequenceUsage {
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
    /// Catalog (database) containing the sequence.
    pub sequence_catalog: Option<String>,
    /// Schema containing the sequence.
    pub sequence_schema: Option<String>,
    /// Name of the sequence.
    pub sequence_name: Option<String>,
}
