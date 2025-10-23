//! Model struct for the `information_schema.routine_privileges` view.
//!
//! This view contains one row for each privilege granted to a routine in the
//! current database.

use diesel::prelude::*;

/// Model struct for `information_schema.routine_privileges`.
///
/// This view contains one row for each privilege granted to a routine in the
/// current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::routine_privileges::routine_privileges)]
pub struct RoutinePrivileges {
    /// User who granted the privilege.
    pub grantor: Option<String>,
    /// User who received the privilege.
    pub grantee: Option<String>,
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
    /// Type of privilege granted.
    pub privilege_type: Option<String>,
    /// Whether the privilege is grantable.
    pub is_grantable: Option<String>,
}
