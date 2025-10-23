//! Submodule providing the `PgPolicy` struct representing a row of the
//! `pg_policies` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_policies` view.
///
/// The `pg_policies` view provides information about row-level security
/// policies defined on tables. Policies control which rows are visible or
/// modifiable for different users.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-policies.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_policies::pg_policies)]
pub struct PgPolicy {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Policy name.
    pub policyname: Option<String>,
    /// Policy type (PERMISSIVE or RESTRICTIVE).
    pub permissive: Option<String>,
    /// Roles to which the policy applies.
    pub roles: Option<Vec<String>>,
    /// Command type the policy applies to.
    pub cmd: Option<String>,
    /// USING clause expression.
    pub qual: Option<String>,
    /// WITH CHECK clause expression.
    pub with_check: Option<String>,
}
