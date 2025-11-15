//! Submodule implementing the `From` trait to convert a
//! `TableTransitiveExtension` into an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule, InternalTrait},
};

use crate::{
    structs::TableTransitiveExtension,
    traits::{TRAIT_MODULE_NAME, TableTransitiveExtensionLike},
};

impl<'table, T> From<TableTransitiveExtension<'table, T>> for InternalModule
where
    T: TableTransitiveExtensionLike + ?Sized,
{
    fn from(transitive_ext: TableTransitiveExtension<'table, T>) -> Self {
        let schema_crate_ref = transitive_ext
            .table
            .table_schema_ref(transitive_ext.workspace)
            .expect("Failed to get the table schema ref for the table transitive extension");
        let trait_name = transitive_ext.table.table_transitive_extension_trait_name();
        let schema_doc_path = transitive_ext.table.table_schema_doc_path();

        let internal_trait: InternalTrait = InternalTrait::from(transitive_ext);
        let auto_blanket = internal_trait.auto_blanket().expect("Failed to generate auto blanket");

        InternalModule::new()
            .public()
            .name(TRAIT_MODULE_NAME)
            .expect("Failed to set the module name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] trait for the {} table.",
                        trait_name, schema_doc_path
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .public()
            .internal_trait(internal_trait)
            .expect("Failed to add the internal trait to module")
            .internal_token(auto_blanket)
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}
