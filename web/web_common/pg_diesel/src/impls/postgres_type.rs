//! Implementations of [`PostgresType`] for database types and columns.
//!
//! This module implements the [`PostgresType`] trait for types that can resolve
//! their PostgreSQL type information. This is used for type mapping and Diesel
//! type generation.
//!
//! ## Implemented Types
//!
//! - [`PgType`](crate::models::PgType): Returns itself (already a type)
//! - [`Column`](crate::models::Column): Queries `pg_type` to resolve the
//!   column's type
//!
//! The trait provides a unified interface for obtaining [`PgType`] information
//! regardless of the source (direct type object or column with type reference).

use crate::{
    models::{Column, PgType},
    traits::PostgresType,
};

impl PostgresType for PgType {
    fn postgres_type(
        &self,
        _conn: &mut diesel::PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        Ok(self.clone())
    }
}

impl PostgresType for Column {
    fn postgres_type(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        self.pg_type(conn)
    }
}
