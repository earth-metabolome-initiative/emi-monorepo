//! Model structs representing rows from PostgreSQL system catalogs.
//!
//! This module provides Diesel-queryable structs for all major PostgreSQL
//! metadata tables and views across three schemas:
//!
//! - [`information_schema`]: SQL-standard metadata views (portable across
//!   databases)
//! - [`pg_catalog`]: PostgreSQL-specific system catalog tables
//! - [`public`]: PostGIS extension tables (geography and geometry columns)
//!
//! Each struct corresponds to a table or view in PostgreSQL and can be queried
//! using Diesel. The structs implement `Queryable`, `QueryableByName`, and
//! `Selectable`.
//!
//! ## Schema Organization
//!
//! The `information_schema` contains ANSI SQL standard views such as `tables`,
//! `columns`, `constraints`, etc. The `pg_catalog` contains PostgreSQL-specific
//! tables like `pg_class`, `pg_attribute`, `pg_proc`, and various statistics
//! views. The `public` schema includes PostGIS extension tables for spatial
//! column metadata.

mod information_schema;
mod pg_catalog;
mod public;
pub use information_schema::*;
pub use pg_catalog::*;
pub use public::*;
