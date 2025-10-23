//! Model struct for the `information_schema.sql_parts` view.
//!
//! This view contains information about major parts of the SQL standard
//! supported by the database system, showing which SQL standard parts are
//! supported and verified.

use diesel::prelude::*;

/// Represents a row from the `information_schema.sql_parts` view.
/// Contains information about major parts of the SQL standard supported by the
/// database system, showing which SQL standard parts are supported and
/// verified.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::sql_parts::sql_parts)]
pub struct SqlParts {
    /// Identifier of the SQL standard part.
    pub feature_id: Option<String>,
    /// Descriptive name of the SQL standard part.
    pub feature_name: Option<String>,
    /// "YES" if the part is supported, "NO" if not.
    pub is_supported: Option<String>,
    /// Name of the conformance test that verifies this part.
    pub is_verified_by: Option<String>,
    /// Additional comments about the part implementation.
    pub comments: Option<String>,
}
