//! RoleTableGrants model representing the information_schema.role_table_grants
//! view.

use diesel::prelude::*;

/// Model struct for `information_schema.role_table_grants`.
///
/// This view contains one row for each table privilege granted to or by a role
/// in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::role_table_grants::role_table_grants)]
pub struct RoleTableGrants {
    /// Role that granted the privilege.
    pub grantor: Option<String>,
    /// Role that received the privilege.
    pub grantee: Option<String>,
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Type of privilege granted.
    pub privilege_type: Option<String>,
    /// Whether the privilege is grantable.
    pub is_grantable: Option<String>,
    /// Whether the privilege applies to the hierarchy.
    pub with_hierarchy: Option<String>,
}
