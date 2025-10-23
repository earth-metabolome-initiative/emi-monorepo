//! Model struct for the `information_schema.role_usage_grants` view.
//!
//! This view contains one row for each usage privilege granted to or by a role
//! in the current database.

use diesel::prelude::*;

/// Model struct for `information_schema.role_usage_grants`.
///
/// This view contains one row for each usage privilege granted to or by a role
/// in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(
    table_name = crate::schema::information_schema::role_usage_grants::role_usage_grants
)]
pub struct RoleUsageGrants {
    /// Role that granted the privilege.
    pub grantor: Option<String>,
    /// Role that received the privilege.
    pub grantee: Option<String>,
    /// Catalog (database) containing the object.
    pub object_catalog: Option<String>,
    /// Schema containing the object.
    pub object_schema: Option<String>,
    /// Name of the object.
    pub object_name: Option<String>,
    /// Type of the object.
    pub object_type: Option<String>,
    /// Type of privilege granted.
    pub privilege_type: Option<String>,
    /// Whether the privilege is grantable.
    pub is_grantable: Option<String>,
}
