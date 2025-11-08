//! Submodule implementing the `From` trait to convert a
//! `TableExtensionAttributes` into an `InternalData`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalData},
};

use crate::{structs::TableExtensionAttributes, traits::TableExtensionAttributesLike};

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalData
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        let display_impl = extension_attributes.display_impl();
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
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .variant(extension_attributes_clone.into())
            .add_trait(display_impl)
            .unwrap()
            .build()
            .unwrap()
    }
}
