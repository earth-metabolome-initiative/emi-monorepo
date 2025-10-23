//! InformationSchemaCatalogName model representing the
//! information_schema.information_schema_catalog_name view.

use diesel::prelude::*;

/// Model struct for `information_schema.information_schema_catalog_name`.
///
/// This view contains the name of the catalog in which the information schema
/// is located.
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Queryable, QueryableByName, Selectable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::information_schema_catalog_name::information_schema_catalog_name)]
pub struct InformationSchemaCatalogName {
    /// Name of the catalog containing the information schema.
    pub catalog_name: Option<String>,
}
