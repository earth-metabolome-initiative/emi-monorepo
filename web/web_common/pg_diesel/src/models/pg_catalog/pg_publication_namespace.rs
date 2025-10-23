//! Submodule providing the `PgPublicationNamespace` struct representing a row
//! of the `pg_publication_namespace` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_publication_namespace` table.
///
/// The `pg_publication_namespace` catalog contains the mapping between
/// publications and schemas. When a publication is defined to include all
/// tables in certain schemas, those associations are recorded here.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-publication-namespace.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_publication_namespace::pg_publication_namespace)]
pub struct PgPublicationNamespace {
    /// OID of the mapping entry.
    pub oid: u32,
    /// OID of the publication.
    pub pnpubid: u32,
    /// OID of the schema.
    pub pnnspid: u32,
}
