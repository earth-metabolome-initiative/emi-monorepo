//! Model struct for the `information_schema.triggered_update_columns` view.
//!
//! This view contains information about columns that are used by triggers for
//! UPDATE events, with one row for each column that can trigger an UPDATE
//! trigger.

use diesel::prelude::*;

/// Represents a row from the `information_schema.triggered_update_columns`
/// view. Contains information about columns that are used by triggers for
/// UPDATE events, with one row for each column that can trigger an UPDATE
/// trigger.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::triggered_update_columns::triggered_update_columns)]
pub struct TriggeredUpdateColumns {
    /// Name of the database (catalog) containing the trigger.
    pub trigger_catalog: Option<String>,
    /// Name of the schema containing the trigger.
    pub trigger_schema: Option<String>,
    /// Name of the trigger.
    pub trigger_name: Option<String>,
    /// Name of the database (catalog) containing the table with the trigger.
    pub event_object_catalog: Option<String>,
    /// Name of the schema containing the table with the trigger.
    pub event_object_schema: Option<String>,
    /// Name of the table that has the trigger.
    pub event_object_table: Option<String>,
    /// Name of the column that can trigger the UPDATE trigger.
    pub event_object_column: Option<String>,
}
