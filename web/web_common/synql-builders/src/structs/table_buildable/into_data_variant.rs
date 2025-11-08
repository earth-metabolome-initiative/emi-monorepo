//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalData`.

use synql_core::structs::{InternalDataVariant, InternalStruct};

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'data, 'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'data, 'table, T>>
    for InternalDataVariant
{
    fn from(buildable: TableBuildable<'data, 'table, T>) -> Self {
        let struct_variant: InternalStruct = buildable.into();
        struct_variant.into()
    }
}
