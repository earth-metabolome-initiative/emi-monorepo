//! Submodule providing a struct [`PgDescription`] representing the
//! `pg_description` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents the `pg_description` system catalog table in `PostgreSQL`.
/// This table stores comments/descriptions for database objects.
#[derive(Clone, Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_description::pg_description)]
pub struct PgDescription {
    /// OID of the object the description is for.
    pub objoid: u32,
    /// Class OID of the object.
    pub classoid: u32,
    /// Object sub-ID (0 if not a sub-object).
    pub objsubid: i32,
    /// The description text.
    pub description: String,
}
