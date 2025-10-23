//! RoleRoutineGrants model representing the
//! information_schema.role_routine_grants view.

use diesel::prelude::*;

/// Model struct for `information_schema.role_routine_grants`.
///
/// This view contains one row for each routine privilege granted to or by a
/// role in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::role_routine_grants::role_routine_grants)]
pub struct RoleRoutineGrants {
    /// Role that granted the privilege.
    pub grantor: Option<String>,
    /// Role that received the privilege.
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
