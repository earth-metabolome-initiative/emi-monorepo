//! Model struct for the `information_schema.sql_implementation_info` view.
//!
//! This view contains information about implementation-specific characteristics
//! of the SQL processor, providing details about database implementation
//! limits, defaults, and behaviors.

use diesel::prelude::*;

/// Represents a row from the `information_schema.sql_implementation_info` view.
/// Contains information about implementation-specific characteristics of the
/// SQL processor, providing details about database implementation limits,
/// defaults, and behaviors.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::sql_implementation_info::sql_implementation_info)]
pub struct SqlImplementationInfo {
    /// Identifier of the implementation information item.
    pub implementation_info_id: Option<String>,
    /// Descriptive name of the implementation information item.
    pub implementation_info_name: Option<String>,
    /// Integer value for the implementation characteristic (if applicable).
    pub integer_value: Option<i32>,
    /// Character value for the implementation characteristic (if applicable).
    pub character_value: Option<String>,
    /// Additional comments about the implementation characteristic.
    pub comments: Option<String>,
}
