//! Submodule providing `PostGIS` support for the webcodegen crate.

pub mod geography_columns;
pub mod geometry_columns;

pub use geography_columns::GeographyColumn;
pub use geometry_columns::GeometryColumn;
