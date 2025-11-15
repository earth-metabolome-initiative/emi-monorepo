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
        let mut builder = InternalTrait::new()
            .public()
            .name(&value.table.table_buildable_key_settable_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
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
            .super_traits(value.table.extended_tables(value.database).iter().map(|table| {
                table
                    .buildable_key_settable_trait_ref(value.workspace)
                    .expect("Failed to get the buildable key settable trait ref for extended table")
                    .into()
            }));

        if !value.table.is_extension(value.database) {
            builder = builder.sized();
        }

        builder.build().expect("Failed to convert internal trait builder into internal trait")
    }
}
