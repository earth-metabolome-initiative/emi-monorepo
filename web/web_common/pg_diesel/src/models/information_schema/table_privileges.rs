//! TablePrivileges model representing the information_schema.table_privileges
//! view.
use diesel::prelude::*;

/// Represents a row from the `information_schema.table_privileges` view.
/// Contains one row for each privilege granted on a table or view to a
/// currently enabled role or granted by a currently enabled role.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::table_privileges::table_privileges)]
pub struct TablePrivileges {
    /// Name of the role that granted the privilege.
    pub grantor: Option<String>,
    /// Name of the role that was granted the privilege.
    pub grantee: Option<String>,
    /// Name of the database (catalog) containing the table.
    pub table_catalog: Option<String>,
    /// Name of the schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Type of privilege (SELECT, INSERT, UPDATE, DELETE, etc.).
    pub privilege_type: Option<String>,
    /// "YES" if the privilege is grantable to others, "NO" otherwise.
    pub is_grantable: Option<String>,
    /// "YES" if the privilege applies to hierarchy, "NO" otherwise.
    pub with_hierarchy: Option<String>,
}
