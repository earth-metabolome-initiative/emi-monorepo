//! Submodule providing PostGIS support for the webcodegen crate.

pub mod geometry_columns;
pub mod geography_columns;

pub use geometry_columns::GeometryColumn;
pub use geography_columns::GeographyColumn;