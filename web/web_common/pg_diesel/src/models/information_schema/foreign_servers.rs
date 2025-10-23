//! Model struct for the `information_schema.foreign_servers` view.
//!
//! This view contains one row for each foreign server in the current database.

use diesel::prelude::*;

/// Model struct for `information_schema.foreign_servers`.
///
/// This view contains one row for each foreign server in the current database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::foreign_servers::foreign_servers)]
pub struct ForeignServers {
    /// Catalog (database) containing the foreign server.
    pub foreign_server_catalog: Option<String>,
    /// Name of the foreign server.
    pub foreign_server_name: Option<String>,
    /// Catalog (database) containing the foreign data wrapper.
    pub foreign_data_wrapper_catalog: Option<String>,
    /// Name of the foreign data wrapper.
    pub foreign_data_wrapper_name: Option<String>,
    /// Type of the foreign server.
    pub foreign_server_type: Option<String>,
    /// Version of the foreign server.
    pub foreign_server_version: Option<String>,
    /// Authorization identifier (owner) of the foreign server.
    pub authorization_identifier: Option<String>,
}
