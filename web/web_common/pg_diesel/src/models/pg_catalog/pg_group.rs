//! Submodule providing the `PgGroup` struct representing a row of the
//! `pg_group` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_group` view.
///
/// The `pg_group` view provides access to information about database role
/// groups. This is a backwards compatibility view for pre-8.1 applications.
/// Modern applications should use pg_roles instead.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-group.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_group::pg_group)]
pub struct PgGroup {
    /// Name of the group role.
    pub groname: Option<String>,
    /// OID of the group role.
    pub grosysid: Option<u32>,
    /// Array of member role OIDs.
    pub grolist: Option<Vec<u32>>,
}
