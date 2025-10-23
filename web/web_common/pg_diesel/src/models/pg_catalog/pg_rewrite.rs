//! Submodule providing the `PgRewrite` struct representing a row of the
//! `pg_rewrite` table in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_rewrite` table.
///
/// The `pg_rewrite` catalog stores rewrite rules for tables and views. Rules
/// allow query modification and are particularly important for views.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-rewrite.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_rewrite::pg_rewrite)]
pub struct PgRewrite {
    /// OID of the rule.
    pub oid: u32,
    /// Name of the rule.
    pub rulename: String,
    /// OID of the table/view.
    pub ev_class: u32,
    /// Event type.
    pub ev_type: String,
    /// Rule firing mode.
    pub ev_enabled: String,
    /// Whether INSTEAD rule.
    pub is_instead: bool,
    /// WHERE condition expression.
    pub ev_qual: String,
    /// Action expression.
    pub ev_action: String,
}
