//! Model struct for the `information_schema.user_mappings` view.
//!
//! This view contains one row for each user mapping defined in the current
//! database that is accessible to the current user, used with foreign data
//! wrappers for user authentication.

use diesel::prelude::*;

/// Represents a row from the `information_schema.user_mappings` view.
/// Contains one row for each user mapping defined in the current database
/// that is accessible to the current user, used with foreign data wrappers
/// for user authentication.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::user_mappings::user_mappings)]
pub struct UserMappings {
    /// Name of the user who owns the user mapping.
    pub authorization_identifier: Option<String>,
    /// Name of the database (catalog) containing the foreign server.
    pub foreign_server_catalog: Option<String>,
    /// Name of the foreign server.
    pub foreign_server_name: Option<String>,
}
