//! Submodule implementing the `From` trait to convert a `TableInsertable` into
//! an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableBuildable,
    traits::{BUILDABLE_MODULE_NAME, TableBuildableLike},
};

impl<'data, 'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'data, 'table, T>>
    for InternalModule<'data>
{
    fn from(value: TableBuildable<'data, 'table, T>) -> Self {
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the buildable module");

        InternalModule::new()
            .name(BUILDABLE_MODULE_NAME)
            .expect("Failed to set buildable module name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] buildable struct for the {} table.",
                        value.table.table_buildable_name(),
                        value.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .data(value.into())
            .expect("Failed to add buildable struct to buildable module")
            .build()
            .expect("Failed to build buildable module")
    }
}
