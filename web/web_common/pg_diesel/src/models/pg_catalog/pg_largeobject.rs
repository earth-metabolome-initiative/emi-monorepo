//! Submodule providing the `PgLargeobject` struct representing a row of the
//! `pg_largeobject` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_largeobject` table.
///
/// The `pg_largeobject` catalog contains the data pages of large objects. Each
/// large object is broken into segments or "pages" small enough to be
/// conveniently stored as rows in pg_largeobject. Each row holds one page
/// of a large object, identified by loid and pageno.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-largeobject.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_largeobject::pg_largeobject)]
#[diesel(primary_key(loid, pageno))]
pub struct PgLargeobject {
    /// OID of the large object.
    pub loid: u32,
    /// Page number within the large object.
    pub pageno: i32,
    /// Data stored in this page.
    pub data: Vec<u8>,
}
