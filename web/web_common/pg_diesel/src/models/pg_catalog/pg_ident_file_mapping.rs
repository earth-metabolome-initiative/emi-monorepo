//! Submodule providing the `PgIdentFileMapping` struct representing a row of
//! the `pg_ident_file_mappings` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_ident_file_mappings` view.
///
/// The `pg_ident_file_mappings` view provides a summary of the contents of the
/// user name mapping configuration file (pg_ident.conf). A row appears for each
/// non-empty, non-comment line in the file.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-ident-file-mappings.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ident_file_mappings::pg_ident_file_mappings)]
pub struct PgIdentFileMapping {
    /// Mapping number.
    pub map_number: Option<i32>,
    /// Name of the file containing this mapping.
    pub file_name: Option<String>,
    /// Line number of this mapping within the file.
    pub line_number: Option<i32>,
    /// Name of the map.
    pub map_name: Option<String>,
    /// System user name.
    pub sys_name: Option<String>,
    /// PostgreSQL user name.
    pub pg_username: Option<String>,
    /// Error message if the mapping is invalid.
    pub error: Option<String>,
}
