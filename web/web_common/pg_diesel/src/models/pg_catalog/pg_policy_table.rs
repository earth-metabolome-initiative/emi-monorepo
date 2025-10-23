//! Submodule providing the `PgPolicyTable` struct representing a row of the
//! `pg_policy` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_policy` table.
///
/// The `pg_policy` catalog stores row-level security policies. Note that this
/// is the underlying table, distinct from the `pg_policies` view which provides
/// a more user-friendly representation.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-policy.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_policy::pg_policy)]
pub struct PgPolicyTable {
    /// OID of the policy.
    pub oid: u32,
    /// Name of the policy.
    pub polname: String,
    /// OID of the table.
    pub polrelid: u32,
    /// Command type.
    pub polcmd: String,
    /// Whether the policy is permissive.
    pub polpermissive: bool,
    /// Role OIDs to which the policy applies.
    pub polroles: Vec<u32>,
    /// USING clause expression.
    pub polqual: Option<String>,
    /// WITH CHECK clause expression.
    pub polwithcheck: Option<String>,
}
