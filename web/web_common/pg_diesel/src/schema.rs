//! Diesel table! macro definitions for PostgreSQL system schemas.
//!
//! This module contains Diesel schema definitions (using the `table!` macro)
//! for all PostgreSQL metadata tables and views. Each submodule corresponds to
//! a PostgreSQL schema:
//!
//! - [`information_schema`]: ANSI SQL standard views for database metadata
//! - [`pg_catalog`]: PostgreSQL system catalog tables and views
//! - [`public`]: PostGIS extension tables for spatial metadata
//!
//! The schema definitions include:
//! - Table names and primary keys
//! - Column names and their Diesel types
//! - Documentation for each table and column
//! - `allow_tables_to_appear_in_same_query!` declarations for joins
//!
//! These schemas are used by the corresponding model structs in the
//! [`models`](crate::models) module.

pub mod information_schema;
pub mod pg_catalog;
pub mod public;
