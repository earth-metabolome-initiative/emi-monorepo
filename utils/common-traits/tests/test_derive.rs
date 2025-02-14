//! Test module to very that the derive macro works as expected.

#[cfg(feature = "derive")]
use common_traits::prelude::{basic, Basic};

#[cfg(feature = "derive")]
#[basic]
/// Simple struct with no generic
pub struct NoGeneric {
    /// A field
    field: i32,
}

#[cfg(feature = "derive")]
#[basic]
/// Struct with a generic
pub struct WithGeneric<T> {
    /// A field
    field: T,
}

#[cfg(feature = "derive")]
#[basic]
/// Enum with no generic
pub enum NoGenericEnum {
    /// A variant
    Variant(i32),
}

#[cfg(feature = "derive")]
#[basic]
/// Enum with a generic
pub enum WithGenericEnum<T> {
    /// A variant
    Variant(T),
}

#[cfg(feature = "derive")]
#[basic]
/// Struct with a generic and a where clause
pub struct WithWhereClause<T>
{
    /// A field
    field: T,
}
