//! Submodule providing the `PgOpclass` struct representing a row of the
//! `pg_opclass` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_opclass` table.
///
/// The `pg_opclass` system catalog defines index access method operator
/// classes. An operator class specifies how a particular data type can be used
/// with an index.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-opclass.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_opclass::pg_opclass)]
pub struct PgOpclass {
    /// OID of the operator class.
    pub oid: u32,
    /// OID of the index access method.
    pub opcmethod: u32,
    /// Name of the operator class.
    pub opcname: String,
    /// OID of the namespace.
    pub opcnamespace: u32,
    /// OID of the owner.
    pub opcowner: u32,
    /// OID of the operator family.
    pub opcfamily: u32,
    /// OID of the data type indexed.
    pub opcintype: u32,
    /// Whether this is the default operator class.
    pub opcdefault: bool,
    /// OID of the data type stored in the index.
    pub opckeytype: u32,
}
