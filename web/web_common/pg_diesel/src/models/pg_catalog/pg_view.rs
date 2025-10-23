//! Model for pg_catalog.pg_views view.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_views::pg_views)]
#[diesel(primary_key(schemaname, viewname))]
/// Represents a row from the `pg_views` view.
pub struct PgView {
    /// Name of schema containing the view
    pub schemaname: Option<String>,
    /// Name of the view
    pub viewname: Option<String>,
    /// Name of the view's owner
    pub viewowner: Option<String>,
    /// View definition (SELECT statement)
    pub definition: Option<String>,
}
