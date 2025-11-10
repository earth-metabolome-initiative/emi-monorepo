//! Submodule implementing the `From` trait to convert a `TableAttributes` into
//! an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableAttributes,
    traits::{TableAttributesLike, table_attributes_like::ATTRIBUTES_MODULE_NAME},
};

impl<'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'table, T>> for InternalModule {
    fn from(attributes: TableAttributes<'table, T>) -> Self {
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalModule::new()
            .name(ATTRIBUTES_MODULE_NAME)
            .expect("Failed to set attributes module name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the attributes enumeration for the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .data(attributes.into())
            .expect("Failed to add attributes enum to attributes module")
            .build()
            .expect("Failed to build attributes module")
    }
}
