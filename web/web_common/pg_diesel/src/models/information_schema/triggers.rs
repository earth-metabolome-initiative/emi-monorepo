//! Model struct for the `information_schema.triggers` view.
//!
//! This view contains one row for each trigger in the current database that the
//! current user owns or has some privilege on, providing comprehensive metadata
//! about triggers including timing, events, and actions.

use std::time::SystemTime;

use diesel::prelude::*;

/// Represents a row from the `information_schema.triggers` view.
/// Contains one row for each trigger in the current database that the current
/// user owns or has some privilege on, providing comprehensive metadata about
/// triggers including timing, events, and actions.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::triggers::triggers)]
pub struct Triggers {
    /// Name of the database (catalog) containing the trigger.
    pub trigger_catalog: Option<String>,
    /// Name of the schema containing the trigger.
    pub trigger_schema: Option<String>,
    /// Name of the trigger.
    pub trigger_name: Option<String>,
    /// Event that activates the trigger (INSERT, UPDATE, DELETE, TRUNCATE).
    pub event_manipulation: Option<String>,
    /// Name of the database (catalog) containing the table with the trigger.
    pub event_object_catalog: Option<String>,
    /// Name of the schema containing the table with the trigger.
    pub event_object_schema: Option<String>,
    /// Name of the table that has the trigger.
    pub event_object_table: Option<String>,
    /// Order of execution when multiple triggers fire on the same event.
    pub action_order: Option<i32>,
    /// Condition that must be met for the trigger to fire (WHEN clause).
    pub action_condition: Option<String>,
    /// SQL statement(s) executed when the trigger fires.
    pub action_statement: Option<String>,
    /// Orientation of the trigger (ROW or STATEMENT).
    pub action_orientation: Option<String>,
    /// Timing of the trigger execution (BEFORE, AFTER, INSTEAD OF).
    pub action_timing: Option<String>,
    /// Name of the transition table for old row values.
    pub action_reference_old_table: Option<String>,
    /// Name of the transition table for new row values.
    pub action_reference_new_table: Option<String>,
    /// Name of the identifier for old row values.
    pub action_reference_old_row: Option<String>,
    /// Name of the identifier for new row values.
    pub action_reference_new_row: Option<String>,
    /// Timestamp when the trigger was created.
    pub created: Option<SystemTime>,
}
