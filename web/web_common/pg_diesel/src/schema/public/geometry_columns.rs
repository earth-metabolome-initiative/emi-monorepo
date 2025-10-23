//! Schema definitions for the `public` schema in a PostgreSQL database with
//! PostGIS extension.

diesel::table! {
    /// `geometry_columns` — a PostGIS metadata view that contains one row for each geometry column
    /// in the database. Provides information about the table, column name, coordinate dimension,
    /// spatial reference system (SRID), and geometry type.
    geometry_columns (f_table_catalog, f_table_schema, f_table_name, f_geometry_column) {
        /// Name of the database (catalog) that contains the table with the geometry column.
        f_table_catalog -> Text,
        /// Name of the schema that contains the table with the geometry column.
        f_table_schema -> Text,
        /// Name of the table that contains the geometry column.
        f_table_name -> Text,
        /// Name of the column that stores geometry values.
        f_geometry_column -> Text,
        /// Coordinate dimension of the geometry column (e.g., 2 for XY, 3 for XYZ).
        coord_dimension -> Integer,
        /// Spatial reference system identifier (SRID) of the geometry column.
        srid -> Integer,
        /// Type of geometry stored in the column (e.g., `POINT`, `LINESTRING`, `POLYGON`).
        r#type -> Text,
    }
}
