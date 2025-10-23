//! Schema definition for the `geography_columns` view in PostGIS.

diesel::table! {
    /// `geography_columns` — a PostGIS metadata view that contains one row for each geography column
    /// in the database. Provides information about the table, column name, coordinate dimension,
    /// spatial reference system (SRID), and geography type.
    public.geography_columns (f_table_catalog, f_table_schema, f_table_name, f_geography_column) {
        /// Name of the database (catalog) that contains the table with the geography column.
        f_table_catalog -> Text,
        /// Name of the schema that contains the table with the geography column.
        f_table_schema -> Text,
        /// Name of the table that contains the geography column.
        f_table_name -> Text,
        /// Name of the column that stores geography values.
        f_geography_column -> Text,
        /// Coordinate dimension of the geography column (e.g., 2 for XY, 3 for XYZ).
        coord_dimension -> Integer,
        /// Spatial reference system identifier (SRID) of the geography column.
        srid -> Integer,
        /// Type of geography stored in the column (e.g., `POINT`, `LINESTRING`, `POLYGON`).
        r#type -> Text,
    }
}
