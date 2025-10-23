//! Model structs for PostGIS extension tables in the `public` schema.
//!
//! This module contains Diesel-queryable structs for PostGIS extension tables
//! that provide metadata about spatial columns (geography and geometry types)
//! in a PostgreSQL database with the PostGIS extension installed.
//!
//! ## Models
//!
//! - [`GeographyColumn`]: Represents a row from `public.geography_columns`
//! - [`GeometryColumn`]: Represents a row from `public.geometry_columns`
//!
//! These models correspond to tables created by the PostGIS extension following
//! the SQL/MM (SQL Multimedia) specification for spatial data.

mod geography_columns;
mod geometry_columns;

pub use geography_columns::GeographyColumn;
pub use geometry_columns::GeometryColumn;
