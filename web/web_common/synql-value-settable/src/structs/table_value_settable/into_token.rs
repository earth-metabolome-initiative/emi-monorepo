//! Submodule implementing the `From` trait to convert a `TableValueSettable`
//! into an `InternalToken`.

use std::rc::Rc;

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::structs::{InternalToken, InternalTrait};

use crate::{structs::TableValueSettable, traits::TableValueSettableLike};

impl<'data, 'table, T> From<TableValueSettable<'data, 'table, T>> for InternalToken<'data>
where
    T: TableValueSettableLike + ?Sized,
    T::DB: InheritableDatabaseLike,
{
    fn from(table_settable: TableValueSettable<'data, 'table, T>) -> Self {
        let trait_ref: InternalTrait<'data> = table_settable.into();
        InternalToken::implements(Rc::from(trait_ref).into()).try_into().unwrap()
    }
}
