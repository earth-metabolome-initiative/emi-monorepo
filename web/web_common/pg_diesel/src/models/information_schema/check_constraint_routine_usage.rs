//! Struct representing a row in the
//! `information_schema.check_constraint_routine_usage` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.check_constraint_routine_usage`
/// table.
///
/// The `check_constraint_routine_usage` view contains one row for each routine
/// (function or procedure) that is used in a check constraint. This view
/// establishes the dependency relationship between check constraints and the
/// functions they call, providing essential information for understanding
/// constraint dependencies and impact analysis when modifying functions.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::check_constraint_routine_usage::check_constraint_routine_usage)]
pub struct CheckConstraintRoutineUsage {
    /// Catalog (database) containing the check constraint.
    /// This identifies the database where the constraint is defined.
    pub constraint_catalog: Option<String>,
    /// Schema containing the check constraint.
    /// This identifies the schema where the constraint is defined.
    pub constraint_schema: Option<String>,
    /// Name of the check constraint.
    /// This is the identifier for the specific check constraint.
    pub constraint_name: Option<String>,
    /// Catalog (database) containing the routine used by the constraint.
    /// This identifies the database where the function or procedure is defined.
    pub specific_catalog: Option<String>,
    /// Schema containing the routine used by the constraint.
    /// This identifies the schema where the function or procedure is defined.
    pub specific_schema: Option<String>,
    /// Specific name of the routine used by the constraint.
    /// This is the identifier for the function or procedure called by the
    /// constraint.
    pub specific_name: Option<String>,
}
