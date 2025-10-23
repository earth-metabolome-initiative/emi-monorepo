//! Model struct for the `information_schema.sql_features` view.
//!
//! This view contains information about SQL features supported by the database
//! system, showing which SQL standard features and sub-features are supported,
//! verified, and any implementation comments.

use diesel::prelude::*;

/// Represents a row from the `information_schema.sql_features` view.
/// Contains information about SQL features supported by the database system,
/// showing which SQL standard features and sub-features are supported,
/// verified, and any implementation comments.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::sql_features::sql_features)]
pub struct SqlFeatures {
    /// Identifier of the feature according to the SQL standard.
    pub feature_id: Option<String>,
    /// Descriptive name of the feature.
    pub feature_name: Option<String>,
    /// Identifier of the sub-feature (if applicable).
    pub sub_feature_id: Option<String>,
    /// Descriptive name of the sub-feature (if applicable).
    pub sub_feature_name: Option<String>,
    /// "YES" if the feature is supported, "NO" if not.
    pub is_supported: Option<String>,
    /// Name of the conformance test that verifies this feature.
    pub is_verified_by: Option<String>,
    /// Additional comments about the feature implementation.
    pub comments: Option<String>,
}
