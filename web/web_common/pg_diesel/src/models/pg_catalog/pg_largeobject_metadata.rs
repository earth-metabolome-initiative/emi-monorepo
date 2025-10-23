//! Submodule providing the `PgLargeobjectMetadatum` struct representing a row
//! of the `pg_largeobject_metadata` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_largeobject_metadata` table.
///
/// The `pg_largeobject_metadata` catalog stores metadata for large objects.
/// Each large object has exactly one entry in this catalog, which holds
/// ownership and access control information. The actual data is stored in
/// `pg_largeobject`.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-largeobject-metadata.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_largeobject_metadata::pg_largeobject_metadata)]
pub struct PgLargeobjectMetadatum {
    /// OID of the large object.
    pub oid: u32,
    /// OID of the owner.
    pub lomowner: u32,
    /// Access privileges.
    pub lomacl: Option<Vec<String>>,
}
