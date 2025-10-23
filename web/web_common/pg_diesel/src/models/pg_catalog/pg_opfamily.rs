//! Submodule providing the `PgOpfamily` struct representing a row of the
//! `pg_opfamily` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_opfamily` table.
///
/// The `pg_opfamily` system catalog defines operator families. An operator
/// family is a collection of operators and support functions that implement the
/// semantics specified for a particular index access method.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-opfamily.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_opfamily::pg_opfamily)]
pub struct PgOpfamily {
    /// OID of the operator family.
    pub oid: u32,
    /// OID of the index access method.
    pub opfmethod: u32,
    /// Name of the operator family.
    pub opfname: String,
    /// OID of the namespace.
    pub opfnamespace: u32,
    /// OID of the owner.
    pub opfowner: u32,
}
