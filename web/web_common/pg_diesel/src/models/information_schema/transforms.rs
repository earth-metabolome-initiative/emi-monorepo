//! Model struct for the `information_schema.transforms` view.
//!
//! This view contains information about transform functions for user-defined
//! types, with one row for each transform function that transforms between a
//! user-defined type and SQL data types.

use diesel::prelude::*;

/// Represents a row from the `information_schema.transforms` view.
/// Contains information about transform functions for user-defined types,
/// with one row for each transform function that transforms between a
/// user-defined type and SQL data types.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::transforms::transforms)]
pub struct Transforms {
    /// Name of the database (catalog) containing the user-defined type.
    pub udt_catalog: Option<String>,
    /// Name of the schema containing the user-defined type.
    pub udt_schema: Option<String>,
    /// Name of the user-defined type.
    pub udt_name: Option<String>,
    /// Name of the database (catalog) containing the transform function.
    pub specific_catalog: Option<String>,
    /// Name of the schema containing the transform function.
    pub specific_schema: Option<String>,
    /// Name of the transform function.
    pub specific_name: Option<String>,
    /// Group name of the transform (typically for related transforms).
    pub group_name: Option<String>,
    /// Type of transform ("FROM SQL" or "TO SQL").
    pub transform_type: Option<String>,
}
