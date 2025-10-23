//! Submodule providing the `PgDbRoleSetting` struct representing a row of the
//! `pg_db_role_setting` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_db_role_setting` table.
///
/// The `pg_db_role_setting` system catalog records the default values that have
/// been configured for run-time configuration variables, for each role and
/// database combination. These settings override the server-wide default
/// values.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-db-role-setting.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_db_role_setting::pg_db_role_setting)]
pub struct PgDbRoleSetting {
    /// OID of the database this setting applies to (0 = all databases).
    pub setdatabase: u32,
    /// OID of the role this setting applies to (0 = all roles).
    pub setrole: u32,
    /// Array of configuration parameter settings.
    pub setconfig: Option<Vec<String>>,
}
