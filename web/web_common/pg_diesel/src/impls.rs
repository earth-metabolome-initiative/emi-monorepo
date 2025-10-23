//! Trait implementations connecting models to the `sql_traits` ecosystem.
//!
//! This module implements various traits from the `sql_traits` crate for the
//! model structs defined in this crate. These implementations enable generic
//! database introspection and code generation.
//!
//! ## Implemented Traits
//!
//! - [`TableLike`](sql_traits::traits::TableLike): Implemented for
//!   [`Table`](crate::models::Table)
//! - [`ColumnLike`](sql_traits::traits::ColumnLike): Implemented for
//!   [`Column`](crate::models::Column)
//! - [`CheckConstraintLike`]: Implemented for
//!   [`CheckConstraint`](crate::models::CheckConstraint)
//! - [`ForeignKeyLike`]: Implemented for
//!   [`KeyColumnUsage`](crate::models::KeyColumnUsage)
//! - [`FunctionLike`]: Implemented for [`PgProc`](crate::models::PgProc)
//! - [`UniqueIndexLike`]: Implemented for [`PgIndex`](crate::models::PgIndex)
//! - [`HasOid`](crate::traits::HasOid): Implemented for various catalog types
//! - [`PostgresType`](crate::traits::PostgresType): Implemented for types that
//!   resolve their Postgres type

mod check_constraint_like;
mod column_like;
mod foreign_key_like;
mod function_like;
mod oid;
mod postgres_type;
mod table_like;
mod unique_index_like;
