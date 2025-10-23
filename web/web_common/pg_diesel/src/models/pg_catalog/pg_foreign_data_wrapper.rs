//! Submodule providing the `PgForeignDataWrapper` struct representing a row of
//! the `pg_foreign_data_wrapper` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_foreign_data_wrapper` table.
///
/// The `pg_foreign_data_wrapper` system catalog stores foreign data wrapper
/// definitions. A foreign data wrapper is a mechanism for accessing external
/// data in SQL tables.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-foreign-data-wrapper.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_foreign_data_wrapper::pg_foreign_data_wrapper)]
pub struct PgForeignDataWrapper {
    /// OID of the foreign data wrapper.
    pub oid: u32,
    /// Name of the foreign data wrapper.
    pub fdwname: String,
    /// OID of the role that owns the foreign data wrapper.
    pub fdwowner: u32,
    /// OID of the handler function.
    pub fdwhandler: u32,
    /// OID of the validator function.
    pub fdwvalidator: u32,
    /// Access privileges.
    pub fdwacl: Option<Vec<String>>,
    /// Foreign data wrapper-specific options.
    pub fdwoptions: Option<Vec<String>>,
}
