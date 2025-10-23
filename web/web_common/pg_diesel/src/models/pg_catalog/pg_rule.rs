//! Submodule providing the `PgRule` struct representing a row of the
//! `pg_rules` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_rules` view.
///
/// The `pg_rules` view provides a user-friendly representation of rules
/// stored in `pg_rewrite`, showing the rule definitions in SQL format.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-rules.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_rules::pg_rules)]
pub struct PgRule {
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Rule name.
    pub rulename: Option<String>,
    /// SQL definition of the rule.
    pub definition: Option<String>,
}
