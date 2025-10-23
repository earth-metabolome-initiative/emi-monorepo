//! Model struct for the `information_schema.sequences` view.
//!
//! This view contains metadata about sequences including data type, precision,
//! range, increment, and cycling behavior.

use diesel::prelude::*;

/// Represents a row from the `information_schema.sequences` view.
/// Contains metadata about sequences including data type, precision, range,
/// increment, and cycling behavior.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::sequences::sequences)]
pub struct Sequences {
    /// Name of the database (catalog) containing the sequence.
    pub sequence_catalog: Option<String>,
    /// Name of the schema containing the sequence.
    pub sequence_schema: Option<String>,
    /// Name of the sequence.
    pub sequence_name: Option<String>,
    /// Data type of the sequence (typically "bigint", "integer", or
    /// "smallint").
    pub data_type: Option<String>,
    /// Precision of the numeric data type (number of significant digits).
    pub numeric_precision: Option<i32>,
    /// Radix of the numeric precision (typically 2 or 10).
    pub numeric_precision_radix: Option<i32>,
    /// Scale of the numeric data type (digits after decimal point).
    pub numeric_scale: Option<i32>,
    /// Start value of the sequence.
    pub start_value: Option<String>,
    /// Minimum value of the sequence.
    pub minimum_value: Option<String>,
    /// Maximum value of the sequence.
    pub maximum_value: Option<String>,
    /// Increment value of the sequence.
    pub increment: Option<String>,
    /// "YES" if the sequence cycles when it reaches its limit, "NO" otherwise.
    pub cycle_option: Option<String>,
}
