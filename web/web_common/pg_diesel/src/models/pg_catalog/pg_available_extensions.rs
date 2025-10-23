//! PostgreSQL available extensions view model.
//!
//! This module provides the `PgAvailableExtensions` struct for working with the
//! `pg_catalog.pg_available_extensions` system view.

/// Represents a row from the `pg_catalog.pg_available_extensions` view.
///
/// Shows extensions that are available for installation.
/// This view provides a summary of PostgreSQL extensions, showing their
/// default versions and whether they are currently installed.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_available_extensions::pg_available_extensions)]
pub struct PgAvailableExtensions {
    /// Name of the extension (primary key).
    pub name: Option<String>,
    /// Default version of the extension.
    pub default_version: Option<String>,
    /// Currently installed version (if any).
    pub installed_version: Option<String>,
    /// Comment describing the extension.
    pub comment: Option<String>,
}
