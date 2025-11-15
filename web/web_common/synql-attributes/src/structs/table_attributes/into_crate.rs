//! Submodule implementing the `From` trait to convert a `TableAttributes` into
//! an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{structs::TableAttributes, traits::TableAttributesLike};

impl<'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'table, T>> for InternalCrate {
    fn from(attributes: TableAttributes<'table, T>) -> Self {
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalCrate::new()
            .name(attributes.table.table_attributes_crate_name())
            .expect("Failed to set attributes crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Crate containing the attributes enumeration for the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .module(attributes.into())
            .expect("Failed to add attributes module to attributes crate")
            .build()
            .expect("Failed to build attributes crate")
    }
}
