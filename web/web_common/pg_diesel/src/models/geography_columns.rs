//! Submodule providing the `GeographyColumn` struct representing a row of the
//! `geography_columns` table.
use diesel::{Queryable, QueryableByName, Selectable};

/// Represents an entry in the `PostGIS` `geography_columns` system table.
///
/// The `geography_columns` table provides metadata about all geography columns
/// stored in the database, including their spatial reference system (SRID)
/// and geography type.
#[derive(Queryable, QueryableByName, Selectable, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::geography_columns)]
pub struct GeographyColumn {
    /// Database/catalog name (often the database name).
    pub f_table_catalog: String,
    /// Schema name containing the table (e.g., `public`).
    pub f_table_schema: String,
    /// Name of the table containing the geography column.
    pub f_table_name: String,
    /// Name of the column that stores the geography data.
    pub f_geography_column: String,
    /// Coordinate dimension (typically `2`, `3`, or `4` for XY, XYZ, XYM,
    /// XYMZ).
    pub coord_dimension: i32,
    /// Spatial Reference System Identifier (SRID) of the geography.
    /// - Common values: `4326` (WGS 84), `3857` (Web Mercator).
    /// - If `-1`, the SRID is unknown.
    pub srid: i32,
    /// The specific geography type stored in this column (e.g., `"POINT"`,
    /// `"LINESTRING"`, `"POLYGON"`).
    pub r#type: String,
}
