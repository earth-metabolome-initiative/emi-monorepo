//! Submodule defining the `ExtensionTable` trait for struct tables.

use std::rc::Rc;

use crate::database::PrimaryKeyLike;

/// Trait marker defining that a table is an extension of another table.
pub trait ExtensionTable<T>
where
    T: PrimaryKeyLike,
    Self: PrimaryKeyLike<PrimaryKey = <T as PrimaryKeyLike>::PrimaryKey>,
{
}

impl<T, A> From<T> for Box<dyn ExtensionTable<A>>
where
    A: PrimaryKeyLike,
    T: ExtensionTable<A> + 'static,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

impl<T, C> ExtensionTable<T> for &C
where
    C: ExtensionTable<T>,
    T: PrimaryKeyLike,
    Self: PrimaryKeyLike<PrimaryKey = <T as PrimaryKeyLike>::PrimaryKey>,
{
}

impl<T, C> ExtensionTable<T> for Box<C>
where
    C: ExtensionTable<T>,
    T: PrimaryKeyLike,
    Self: PrimaryKeyLike<PrimaryKey = <T as PrimaryKeyLike>::PrimaryKey>,
{
}

impl<T, C> ExtensionTable<T> for Rc<C>
where
    C: ExtensionTable<T>,
    T: PrimaryKeyLike,
    Self: PrimaryKeyLike<PrimaryKey = <T as PrimaryKeyLike>::PrimaryKey>,
{
}
