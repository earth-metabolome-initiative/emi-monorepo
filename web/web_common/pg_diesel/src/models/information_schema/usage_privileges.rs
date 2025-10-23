//! Model struct for the `information_schema.usage_privileges` view.
//!
//! This view contains one row for each USAGE privilege granted on a schema,
//! domain, collation, character set, or translation to a currently enabled
//! role.

use diesel::prelude::*;

/// Represents a row from the `information_schema.usage_privileges` view.
/// Contains one row for each USAGE privilege granted on a schema, domain,
/// collation, character set, or translation to a currently enabled role.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::usage_privileges::usage_privileges)]
pub struct UsagePrivileges {
    /// Name of the role that granted the privilege.
    pub grantor: Option<String>,
    /// Name of the role that was granted the privilege.
    pub grantee: Option<String>,
    /// Name of the database (catalog) containing the object.
    pub object_catalog: Option<String>,
    /// Name of the schema containing the object.
    pub object_schema: Option<String>,
    /// Name of the object.
    pub object_name: Option<String>,
    /// Type of object (SCHEMA, DOMAIN, COLLATION, CHARACTER SET, TRANSLATION).
    pub object_type: Option<String>,
    /// Type of privilege (always USAGE for this view).
    pub privilege_type: Option<String>,
    /// "YES" if the privilege is grantable to others, "NO" otherwise.
    pub is_grantable: Option<String>,
}
