//! Model struct for the `information_schema.foreign_data_wrappers` view.
//!
//! This view contains one row for each foreign data wrapper in the current
//! database.

use diesel::prelude::*;

/// Model struct for `information_schema.foreign_data_wrappers`.
///
/// This view contains one row for each foreign data wrapper in the current
/// database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::foreign_data_wrappers::foreign_data_wrappers)]
pub struct ForeignDataWrappers {
    /// Catalog (database) containing the foreign data wrapper.
    pub foreign_data_wrapper_catalog: Option<String>,
    /// Name of the foreign data wrapper.
    pub foreign_data_wrapper_name: Option<String>,
    /// Authorization identifier (owner) of the foreign data wrapper.
    pub authorization_identifier: Option<String>,
    /// Name of the library that implements the foreign data wrapper.
    pub library_name: Option<String>,
    /// Language used to implement the foreign data wrapper.
    pub foreign_data_wrapper_language: Option<String>,
}
