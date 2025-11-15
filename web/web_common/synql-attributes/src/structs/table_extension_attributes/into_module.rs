//! Submodule implementing the `From` trait to convert a
//! `TableExtensionAttributes` into an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableExtensionAttributes,
    traits::{
        TableExtensionAttributesLike,
        table_extension_attributes_like::EXTENSION_ATTRIBUTES_MODULE_NAME,
    },
};

impl<'table, T: TableExtensionAttributesLike + ?Sized> From<TableExtensionAttributes<'table, T>>
    for InternalModule
{
    fn from(extension_attributes: TableExtensionAttributes<'table, T>) -> Self {
        let schema_crate_ref = extension_attributes
            .table
            .table_schema_ref(extension_attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalModule::new()
            .name(EXTENSION_ATTRIBUTES_MODULE_NAME)
            .expect("Failed to set extension attributes module name")
            .public()
            .documentation(Documentation::new().documentation(&format!(
                "Submodule providing the extension attributes enumeration for the {} table.",
                extension_attributes.table.table_schema_doc_path()
            )).unwrap().internal_dependency(schema_crate_ref).build().unwrap())
            .data(extension_attributes.into())
            .expect("Failed to add extension attributes enum to extension attributes module")
            .build()
            .unwrap()
    }
}
