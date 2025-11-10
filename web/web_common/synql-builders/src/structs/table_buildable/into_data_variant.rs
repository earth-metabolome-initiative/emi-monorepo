//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalData`.

use synql_core::structs::{InternalDataVariant, InternalStruct};

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'table, T>>
    for InternalDataVariant
{
    fn from(buildable: TableBuildable<'table, T>) -> Self {
        let struct_variant: InternalStruct = buildable.into();
        struct_variant.into()
    }
}
