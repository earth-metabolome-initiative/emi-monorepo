//! Model struct for the `information_schema.user_mapping_options` view.
//!
//! This view contains one row for each option of a user mapping accessible to
//! the current user, used with foreign data wrappers to configure connection
//! and authentication options.

use diesel::prelude::*;

/// Represents a row from the `information_schema.user_mapping_options` view.
/// Contains one row for each option of a user mapping accessible to the current
/// user, used with foreign data wrappers to configure connection and
/// authentication options.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::user_mapping_options::user_mapping_options)]
pub struct UserMappingOptions {
    /// Name of the user who owns the user mapping.
    pub authorization_identifier: Option<String>,
    /// Name of the database (catalog) containing the foreign server.
    pub foreign_server_catalog: Option<String>,
    /// Name of the foreign server.
    pub foreign_server_name: Option<String>,
    /// Name of the option.
    pub option_name: Option<String>,
    /// Value of the option.
    pub option_value: Option<String>,
}
