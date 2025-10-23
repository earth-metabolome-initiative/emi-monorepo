//! PostgreSQL available extension versions view model.
//!
//! This module provides the `PgAvailableExtensionVersions` struct for working
//! with the `pg_catalog.pg_available_extension_versions` system view.

/// Represents a row from the `pg_catalog.pg_available_extension_versions` view.
///
/// Shows all extension versions that are available for installation.
/// This view provides information about PostgreSQL extensions that can be
/// installed in the database, including their versions and properties.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_available_extension_versions::pg_available_extension_versions)]
pub struct PgAvailableExtensionVersions {
    /// Name of the extension (part of composite primary key).
    pub name: Option<String>,
    /// Version string of the extension (part of composite primary key).
    pub version: Option<String>,
    /// Whether this version is currently installed.
    pub installed: Option<bool>,
    /// Whether installation requires superuser privileges.
    pub superuser: Option<bool>,
    /// Whether the extension is trusted (can be installed by non-superusers).
    pub trusted: Option<bool>,
    /// Whether the extension can be relocated to another schema.
    pub relocatable: Option<bool>,
    /// Default schema for the extension.
    pub schema: Option<String>,
    /// Array of extensions that this extension requires.
    pub requires: Option<Vec<String>>,
    /// Comment describing the extension.
    pub comment: Option<String>,
}
