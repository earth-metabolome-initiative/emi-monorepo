//! Model struct for the `information_schema.foreign_table_options` view.
//!
//! This view contains one row for each option of a foreign table in the current
//! database.

use diesel::prelude::*;

/// Model struct for `information_schema.foreign_table_options`.
///
/// This view contains one row for each option of a foreign table in the current
/// database.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::foreign_table_options::foreign_table_options)]
pub struct ForeignTableOptions {
    /// Catalog (database) containing the foreign table.
    pub foreign_table_catalog: Option<String>,
    /// Schema containing the foreign table.
    pub foreign_table_schema: Option<String>,
    /// Name of the foreign table.
    pub foreign_table_name: Option<String>,
    /// Name of the option.
    pub option_name: Option<String>,
    /// Value of the option.
    pub option_value: Option<String>,
}
