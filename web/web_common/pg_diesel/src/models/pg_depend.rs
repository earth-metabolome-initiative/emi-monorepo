//! Submodule providing a struct [`PgDepend`] representing the `pg_depend`
//! table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents the `pg_depend` system catalog table in `PostgreSQL`.
/// This table stores information about the dependency relationships between
/// database objects.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_depend)]
pub struct PgDepend {
    /// The OID of the dependent object.
    pub classid: u32,
    /// The OID of the dependent object.
    pub objid: u32,
    /// The sub-ID of the dependent object (0 if not applicable).
    pub objsubid: i32,
    /// The OID of the referenced class.
    pub refclassid: u32,
    /// The OID of the referenced object.
    pub refobjid: u32,
    /// The sub-ID of the referenced object (0 if not applicable).
    pub refobjsubid: i32,
    /// The dependency type.
    pub deptype: String,
}
