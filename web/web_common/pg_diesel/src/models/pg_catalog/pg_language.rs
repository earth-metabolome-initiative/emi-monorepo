//! Submodule providing the `PgLanguage` struct representing a row of the
//! `pg_language` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_language` table.
///
/// The `pg_language` system catalog registers languages in which you can write
/// functions or stored procedures. PostgreSQL has four built-in languages: SQL,
/// C, internal, and plpgsql.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-language.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_language::pg_language)]
pub struct PgLanguage {
    /// OID of the language.
    pub oid: u32,
    /// Name of the language.
    pub lanname: String,
    /// OID of the role that owns the language.
    pub lanowner: u32,
    /// Whether this is a procedural language.
    pub lanispl: bool,
    /// Whether this is a trusted language.
    pub lanpltrusted: bool,
    /// OID of the call handler function.
    pub lanplcallfoid: u32,
    /// OID of the inline handler function.
    pub laninline: u32,
    /// OID of the validator function.
    pub lanvalidator: u32,
    /// Access privileges.
    pub lanacl: Option<Vec<String>>,
}
