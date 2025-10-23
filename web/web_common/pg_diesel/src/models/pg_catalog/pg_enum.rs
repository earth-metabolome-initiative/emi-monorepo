//! Submodule providing the `PgEnum` struct, which represents a `PostgreSQL`
//! enum type.
use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a `PostgreSQL` enum type.
///
/// This struct maps to the `pg_enum` system catalog table in `PostgreSQL`,
/// which stores metadata about enum types. Each instance of `PgEnum`
/// corresponds to a single enum value in the database.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-enum.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_enum::pg_enum)]
pub struct PgEnum {
    /// The OID of the enum value.
    pub oid: u32,
    /// The OID of the enum type.
    pub enumtypid: u32,
    /// The sort order of the enum value.
    pub enumsortorder: f32,
    /// The label of the enum value.
    pub enumlabel: String,
}
