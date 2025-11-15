//! Submodule implementing the `From` trait to convert a
//! `TableExtensionAttributes` into an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{structs::TableExtensionAttributes, traits::TableExtensionAttributesLike};

impl<'table, T: TableExtensionAttributesLike + ?Sized> From<TableExtensionAttributes<'table, T>>
    for InternalCrate
{
    fn from(extension_attributes: TableExtensionAttributes<'table, T>) -> Self {
        let schema_crate_ref = extension_attributes
            .table
            .table_schema_ref(extension_attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalCrate::new()
            .name(extension_attributes.table.table_extension_attributes_crate_name())
            .expect("Failed to set extension attributes crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Crate containing the extension attributes enumeration for the {} table.",
                        extension_attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .module(extension_attributes.into())
            .expect("Failed to add extension attributes module to extension attributes crate")
            .build()
            .unwrap()
    }
}
