//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalStruct`.

use synql_core::{prelude::Builder, structs::InternalStruct};

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'table, T>> for InternalStruct {
    fn from(buildable: TableBuildable<'table, T>) -> Self {
        InternalStruct::new()
            .attributes(
                buildable
                    .table
                    .extended_tables(buildable.database)
                    .into_iter()
                    .map(|t| t.builder_extension_attribute(buildable.workspace)),
            )
            .unwrap()
            .attribute(buildable.table.insertable_attribute(buildable.workspace))
            .unwrap()
            .build()
            .unwrap()
    }
}
