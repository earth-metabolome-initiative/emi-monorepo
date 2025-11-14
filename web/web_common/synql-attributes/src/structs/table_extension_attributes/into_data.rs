//! Submodule implementing the `From` trait to convert a
//! `TableExtensionAttributes` into an `InternalData`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalData},
};

use crate::{structs::TableExtensionAttributes, traits::TableExtensionAttributesLike};

impl<'table, T: TableExtensionAttributesLike + ?Sized> From<TableExtensionAttributes<'table, T>>
    for InternalData
{
    fn from(extension_attributes: TableExtensionAttributes<'table, T>) -> Self {
        let extension_attributes_clone = extension_attributes.clone();
        let schema_crate_ref = extension_attributes
            .table
            .table_schema_ref(extension_attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");

        InternalData::new()
            .name(extension_attributes.table.table_extension_attributes_name())
            .expect("Failed to set extension attributes enum name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Enumeration of the extension attributes of the {} table.",
                        extension_attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .variant(extension_attributes_clone.into())
            .add_trait(extension_attributes.display_impl())
            .unwrap()
            .add_traits(extension_attributes.from_impls())
            .unwrap()
            .build()
            .unwrap()
    }
}
