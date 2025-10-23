//! Model struct for the `information_schema.sql_sizing` view.
//!
//! This view contains information about SQL sizing limits supported by the
//! database system, showing maximum values for various SQL constructs and
//! implementation limits.

use diesel::prelude::*;

/// Represents a row from the `information_schema.sql_sizing` view.
/// Contains information about SQL sizing limits supported by the database
/// system, showing maximum values for various SQL constructs and implementation
/// limits.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::sql_sizing::sql_sizing)]
pub struct SqlSizing {
    /// Identifier of the sizing limit.
    pub sizing_id: Option<i32>,
    /// Descriptive name of the sizing limit.
    pub sizing_name: Option<String>,
    /// Maximum supported value for this sizing limit.
    pub supported_value: Option<i32>,
    /// Additional comments about the sizing limit.
    pub comments: Option<String>,
}
