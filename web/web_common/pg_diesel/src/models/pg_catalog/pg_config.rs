//! Submodule providing the `PgConfig` struct representing a row of the
//! `pg_config` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_config` view.
///
/// The `pg_config` view provides compile-time configuration information about
/// the PostgreSQL installation. Each row contains a configuration parameter
/// name and its corresponding value. This includes paths to installation
/// directories, version information, and other build-time settings.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-config.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_config::pg_config)]
pub struct PgConfig {
    /// Name of the configuration parameter.
    pub name: Option<String>,
    /// Current value of the configuration parameter.
    pub setting: Option<String>,
}
