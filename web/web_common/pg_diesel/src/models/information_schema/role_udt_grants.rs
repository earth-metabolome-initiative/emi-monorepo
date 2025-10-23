//! RoleUdtGrants model representing the information_schema.role_udt_grants
//! view.

use diesel::prelude::*;

/// Model struct for `information_schema.role_udt_grants`.
///
/// This view contains one row for each user-defined type privilege granted to
/// or by a role in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::role_udt_grants::role_udt_grants)]
pub struct RoleUdtGrants {
    /// Role that granted the privilege.
    pub grantor: Option<String>,
    /// Role that received the privilege.
    pub grantee: Option<String>,
    /// Catalog (database) containing the user-defined type.
    pub udt_catalog: Option<String>,
    /// Schema containing the user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    pub udt_name: Option<String>,
    /// Type of privilege granted.
    pub privilege_type: Option<String>,
    /// Whether the privilege is grantable.
    pub is_grantable: Option<String>,
}
