//! Submodule defining the `FromExtension` trait, which is a restriction over
//! the `From` trait to be implemented by types which can be created from an
//! extension of a table or an associated struct thereof.

use common_traits::{builder::EmptyTuple, prelude::BuilderError};

use crate::database::{InsertError, TableField};

/// This trait is useful to generically convert extension types into descendant
/// types, such as the error associated to the child table should implement this
/// trait for the error associated to the parent table.
pub trait FromExtension<T> {
    /// Converts the given extension type into `Self`.
    fn from_extension(extension: T) -> Self;
}

impl<X, Y> FromExtension<InsertError<X>> for InsertError<Y>
where
    Y: FromExtension<X>,
    X: TableField,
    Y: TableField<Table = X::Table>,
{
    fn from_extension(err: InsertError<X>) -> Self {
        err.into_field_name(Y::from_extension)
    }
}

impl<Y> FromExtension<BuilderError<EmptyTuple>> for InsertError<Y>
where
    Y: FromExtension<EmptyTuple> + TableField,
{
    fn from_extension(err: BuilderError<EmptyTuple>) -> Self {
        err.into_field_name(Y::from_extension).into()
    }
}
