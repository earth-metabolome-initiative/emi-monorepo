//! Struct representing a row in the `information_schema.column_privileges`
//! table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.column_privileges` table.
///
/// The `column_privileges` view contains one row for each privilege granted
/// on a column to a user or role. This provides access control information
/// for column-level security.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::column_privileges::column_privileges)]
pub struct ColumnPrivilege {
    /// Name of the user who granted the privilege.
    pub grantor: Option<String>,
    /// Name of the user or role to whom the privilege was granted.
    pub grantee: Option<String>,
    /// Catalog (database) containing the table.
    pub table_catalog: Option<String>,
    /// Schema containing the table.
    pub table_schema: Option<String>,
    /// Name of the table.
    pub table_name: Option<String>,
    /// Name of the column.
    pub column_name: Option<String>,
    /// Type of privilege granted.
    pub privilege_type: Option<String>,
    /// Whether the privilege is grantable.
    pub is_grantable: Option<String>,
}
