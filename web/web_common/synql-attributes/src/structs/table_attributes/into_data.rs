//! Submodule implementing the `From` trait to convert a `TableAttributes` into
//! an `InternalData`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalData},
};

use crate::{structs::TableAttributes, traits::TableAttributesLike};

impl<'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'table, T>> for InternalData {
    fn from(attributes: TableAttributes<'table, T>) -> Self {
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalData::new()
            .name(attributes.table.table_attributes_name())
            .expect("Failed to set attributes enum name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Enumeration of the attributes of the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .variant(attributes.into())
            .add_trait(attributes.display_impl())
            .unwrap()
            .add_traits(attributes.from_extension_impl())
            .unwrap()
            .build()
            .expect("Failed to build attributes enum")
    }
}
