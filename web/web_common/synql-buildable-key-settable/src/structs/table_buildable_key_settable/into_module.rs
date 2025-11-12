//! Submodule implementing the `From` trait to convert a
//! `TableBuildableKeySettable` into an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableBuildableKeySettable,
    traits::{TRAIT_MODULE_NAME, TableBuildableKeySettableLike},
};

impl<'table, T> From<TableBuildableKeySettable<'table, T>> for InternalModule
where
    T: TableBuildableKeySettableLike + ?Sized,
{
    fn from(value: TableBuildableKeySettable<'table, T>) -> Self {
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        InternalModule::new()
            .public()
            .name(TRAIT_MODULE_NAME)
            .expect("Failed to set the module name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] trait for the {} table.",
                        value.table.table_buildable_key_settable_trait_name(),
                        value.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .public()
            .internal_trait(value.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}
