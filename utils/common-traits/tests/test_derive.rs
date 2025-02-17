//! Test module to very that the derive macro works as expected.

use common_traits::prelude::{basic, Basic};

#[basic]
/// Simple struct with no generic
pub struct NoGeneric {
    /// A field
    field: i32,
}

#[basic]
/// Struct with a generic
pub struct WithGeneric<T> {
    /// A field
    field: T,
}

#[basic]
/// Enum with no generic
pub enum NoGenericEnum {
    /// A variant
    Variant(i32),
}

#[basic]
/// Enum with a generic
pub enum WithGenericEnum<T> {
    /// A variant
    Variant(T),
}

#[basic]
/// Struct with a generic and a where clause
pub struct WithWhereClause<T> {
    /// A field
    field: T,
}
