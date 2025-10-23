//! Model for pg_catalog.pg_wait_events view.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_wait_events::pg_wait_events)]
#[diesel(primary_key(wait_type, name))]
/// Represents a row from the `pg_wait_events` view.
pub struct PgWaitEvent {
    /// Type of wait event
    pub wait_type: Option<String>,
    /// Name of the wait event
    pub name: Option<String>,
    /// Description of the wait event
    pub description: Option<String>,
}
