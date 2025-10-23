//! Submodule providing the `PgPublicationRel` struct representing a row of the
//! `pg_publication_rel` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_publication_rel` table.
///
/// The `pg_publication_rel` catalog contains the mapping between publications
/// and relations (tables). Each entry indicates that a specific table is
/// included in a publication, optionally with a row filter.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-publication-rel.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_publication_rel::pg_publication_rel)]
pub struct PgPublicationRel {
    /// OID of the mapping entry.
    pub oid: u32,
    /// OID of the publication.
    pub prpubid: u32,
    /// OID of the relation.
    pub prrelid: u32,
    /// Optional WHERE clause for row filtering.
    pub prqual: Option<String>,
}
