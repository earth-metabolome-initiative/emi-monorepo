//! Model struct for the `information_schema.enabled_roles` view.
//!
//! This view contains one row for each role that is enabled for the current
//! user.

use diesel::prelude::*;

/// Model struct for `information_schema.enabled_roles`.
///
/// This view contains one row for each role that is enabled for the current
/// user.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::enabled_roles::enabled_roles)]
pub struct EnabledRoles {
    /// Name of the enabled role.
    pub role_name: Option<String>,
}
