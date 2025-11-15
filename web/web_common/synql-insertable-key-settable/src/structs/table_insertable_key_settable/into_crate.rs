//! Submodule implementing the `From` trait to convert a
//! `TableInsertableKeySettable` into an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableInsertableKeySettable,
    traits::{TRAIT_MODULE_NAME, TableInsertableKeySettableLike},
};

impl<'table, T> From<TableInsertableKeySettable<'table, T>> for InternalCrate
where
    T: TableInsertableKeySettableLike + ?Sized,
{
    fn from(value: TableInsertableKeySettable<'table, T>) -> Self {
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalCrate::new()
            .name(value.table.table_insertable_key_settable_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Crate providing the [`{table_insertable_key_settable_trait_name}`](crate::{TRAIT_MODULE_NAME}::{table_insertable_key_settable_trait_name}) trait for the {} table.",
                        value.table.table_schema_doc_path(),
                        table_insertable_key_settable_trait_name=value.table.table_insertable_key_settable_trait_name(),
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .module(value.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
