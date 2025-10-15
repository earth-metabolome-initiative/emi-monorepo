//! Submodule implementing the `TableSyn` trait, which provides methods to
//! facilitate the rust code generation starting from a SQL table
//! representation, building on top of the [`TableLike`](sql_traits::traits::TableLike) trait
//! and the traits from the [`sql_relations`](sql_relations) crate.

/// Trait implemented by types that represent SQL tables and can be used to
/// generate Rust code for them.
pub trait TableSyn: TableLike {}