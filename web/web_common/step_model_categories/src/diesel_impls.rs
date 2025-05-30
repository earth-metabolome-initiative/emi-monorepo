#![cfg(feature = "diesel")]
//! Submodule providing the diesel implementations for instrument categories.

mod postgres;
mod sqlite;

/// The `StepModelCategory` SQL type.
#[derive(
    Debug, Clone, Copy, Default, diesel::query_builder::QueryId, diesel::sql_types::SqlType,
)]
#[cfg_attr(
    all(feature = "postgres", not(feature = "diesel_pgrx")),
    diesel(postgres_type(oid = 25, array_oid = 1009))
)]
#[cfg_attr(
    all(feature = "postgres", feature = "diesel_pgrx"),
    diesel(postgres_type(name = "stepmodelcategory"))
)]
#[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "Text")))]
pub struct StepModelCategory;
