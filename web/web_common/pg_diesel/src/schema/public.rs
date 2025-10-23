//! Diesel schema definitions for PostGIS extension tables in the `public`
//! schema.
//!
//! This module contains Diesel table! definitions for PostGIS extension tables
//! that provide metadata about spatial columns (geography and geometry types)
//! in a PostgreSQL database with the PostGIS extension installed.
//!
//! ## Covered Tables
//!
//! - [`geography_columns`]: Metadata about geography columns
//! - [`geometry_columns`]: Metadata about geometry columns
//!
//! These tables are created by the PostGIS extension and follow the SQL/MM (SQL
//! Multimedia) specification for spatial data.

pub mod geography_columns;
pub mod geometry_columns;
