//! Model struct for the `information_schema.schemata` view.
//!
//! This view contains metadata about database schemas including catalog, schema
//! name, owner, and default character set information.

use diesel::prelude::*;

/// Represents a row from the `information_schema.schemata` view.
/// Contains metadata about database schemas including catalog, schema name,
/// owner, and default character set information.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::schemata::schemata)]
pub struct Schemata {
    /// Name of the database (catalog) containing the schema.
    pub catalog_name: String,
    /// Name of the schema.
    pub schema_name: String,
    /// Name of the user who owns the schema.
    pub schema_owner: String,
    /// Name of the default character set for the schema.
    pub default_character_set_catalog: Option<String>,
    /// Schema containing the default character set.
    pub default_character_set_schema: Option<String>,
    /// Name of the default character set.
    pub default_character_set_name: Option<String>,
    /// SQL path for the schema (typically NULL in PostgreSQL).
    pub sql_path: Option<String>,
}
