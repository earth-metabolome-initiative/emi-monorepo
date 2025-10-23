//! Model struct for the `information_schema.udt_privileges` view.
//!
//! This view contains one row for each privilege granted on a user-defined type
//! to a currently enabled role or granted by a currently enabled role.

use diesel::prelude::*;

/// Represents a row from the `information_schema.udt_privileges` view.
/// Contains one row for each privilege granted on a user-defined type to a
/// currently enabled role or granted by a currently enabled role.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::udt_privileges::udt_privileges)]
pub struct UdtPrivileges {
    /// Name of the role that granted the privilege.
    pub grantor: Option<String>,
    /// Name of the role that was granted the privilege.
    pub grantee: Option<String>,
    /// Name of the database (catalog) containing the user-defined type.
    pub udt_catalog: Option<String>,
    /// Name of the schema containing the user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    pub udt_name: Option<String>,
    /// Type of privilege (USAGE, etc.).
    pub privilege_type: Option<String>,
    /// "YES" if the privilege is grantable to others, "NO" otherwise.
    pub is_grantable: Option<String>,
}
