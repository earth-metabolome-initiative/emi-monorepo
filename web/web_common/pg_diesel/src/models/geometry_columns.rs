//! Submodule providing the `GeometryColumn` struct representing a row of the
//! `geometry_columns` table.
use diesel::{Queryable, QueryableByName, Selectable};

/// Represents an entry in the `PostGIS` `geometry_columns` system table.
///
/// The `geometry_columns` table provides metadata about all geometry columns
/// stored in the database, including their spatial reference system (SRID)
/// and geometry type.
#[derive(Queryable, QueryableByName, Selectable, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::geometry_columns)]
pub struct GeometryColumn {
    /// Database/catalog name (often the database name).
    pub f_table_catalog: String,
    /// Schema name containing the table (e.g., `public`).
    pub f_table_schema: String,
    /// Name of the table containing the geometry column.
    pub f_table_name: String,
    /// Name of the column that stores the geometry data.
    pub f_geometry_column: String,
    /// Coordinate dimension (typically `2`, `3`, or `4` for XY, XYZ, XYM,
    /// XYMZ).
    pub coord_dimension: i32,
    /// Spatial Reference System Identifier (SRID) of the geometry.
    /// - Common values: `4326` (WGS 84), `3857` (Web Mercator).
    /// - If `-1`, the SRID is unknown.
    pub srid: i32,
    /// The specific geometry type stored in this column (e.g., `"POINT"`,
    /// `"LINESTRING"`, `"POLYGON"`).
    pub r#type: String,
}
