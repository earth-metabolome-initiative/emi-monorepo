//! Submodule providing the `PgFileSetting` struct representing a row of the
//! `pg_file_settings` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_file_settings` view.
///
/// The `pg_file_settings` view provides a summary of the contents of the
/// server's configuration files. A row appears in this view for each setting
/// that appears in the configuration files.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-file-settings.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_file_settings::pg_file_settings)]
pub struct PgFileSetting {
    /// Configuration file where this setting is defined.
    pub sourcefile: Option<String>,
    /// Line number within the source file.
    pub sourceline: Option<i32>,
    /// Sequence number showing order of processing.
    pub seqno: Option<i32>,
    /// Name of the configuration parameter.
    pub name: Option<String>,
    /// Value of the configuration parameter.
    pub setting: Option<String>,
    /// Whether the setting was successfully applied.
    pub applied: Option<bool>,
    /// Error message if the setting failed to apply.
    pub error: Option<String>,
}
