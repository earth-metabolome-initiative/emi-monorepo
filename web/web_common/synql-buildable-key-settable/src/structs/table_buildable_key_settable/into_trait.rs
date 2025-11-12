//! Submodule implementing the `From` trait to convert a
//! `TableBuildableKeySettable` into an `InternalTrait`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalTrait},
};

use crate::{structs::TableBuildableKeySettable, traits::TableBuildableKeySettableLike};

impl<'table, T> From<TableBuildableKeySettable<'table, T>> for InternalTrait
where
    T: TableBuildableKeySettableLike + ?Sized,
{
    fn from(value: TableBuildableKeySettable<'table, T>) -> Self {
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalTrait::new()
            .public()
            .name(value.table.table_buildable_key_settable_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Trait providing methods to set the key values of a {} table.",
                        value.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .methods(value.buildable_key_setter_methods())
            .expect("Failed to set the internal trait methods")
            .sized()
            .unwrap()
            .build()
            .expect("Failed to convert internal trait builder into internal trait")
    }
}
