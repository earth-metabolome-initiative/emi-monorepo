//! Submodule providing the `PgParameterAcl` struct representing a row of the
//! `pg_parameter_acl` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_parameter_acl` table.
///
/// The `pg_parameter_acl` catalog stores access privileges for configuration
/// parameters, allowing fine-grained control over who can SET various
/// parameters.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-parameter-acl.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_parameter_acl::pg_parameter_acl)]
pub struct PgParameterAcl {
    /// OID of the parameter ACL entry.
    pub oid: u32,
    /// Name of the configuration parameter.
    pub parname: String,
    /// Access privileges.
    pub paracl: Option<Vec<String>>,
}
