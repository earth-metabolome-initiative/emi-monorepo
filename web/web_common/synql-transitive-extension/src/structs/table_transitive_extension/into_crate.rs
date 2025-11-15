//! Submodule implementing the `From` trait to convert a
//! `TableTransitiveExtension` into an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableTransitiveExtension,
    traits::{TRAIT_MODULE_NAME, TableTransitiveExtensionLike},
};

impl<'table, T> From<TableTransitiveExtension<'table, T>> for InternalCrate
where
    T: TableTransitiveExtensionLike + ?Sized,
{
    fn from(transitive_ext: TableTransitiveExtension<'table, T>) -> Self {
        let schema_crate_ref = transitive_ext
            .table
            .table_schema_ref(transitive_ext.workspace)
            .expect("Failed to get the table schema ref for the table transitive extension");
        InternalCrate::new()
            .name(transitive_ext.table.table_transitive_extension_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Crate providing the [`{trait_name}`](crate::{TRAIT_MODULE_NAME}::{trait_name}) trait for the {} table.",
                        transitive_ext.table.table_schema_doc_path(),
                        trait_name = transitive_ext.table.table_transitive_extension_trait_name(),
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .module(transitive_ext.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
