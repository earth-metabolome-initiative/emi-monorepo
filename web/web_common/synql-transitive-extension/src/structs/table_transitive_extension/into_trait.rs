//! Submodule implementing the `From` trait to convert a
//! `TableTransitiveExtension` into an `InternalTrait`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalTrait},
    utils::generic_type,
};

use crate::{structs::TableTransitiveExtension, traits::TableTransitiveExtensionLike};

impl<'table, T> From<TableTransitiveExtension<'table, T>> for InternalTrait
where
    T: TableTransitiveExtensionLike + ?Sized,
{
    fn from(transitive_ext: TableTransitiveExtension<'table, T>) -> Self {
        let schema_crate_ref = transitive_ext
            .table
            .table_schema_ref(transitive_ext.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        let extensin_of = transitive_ext.workspace.external_trait("ExtensionOf").unwrap();
        InternalTrait::new()
            .public()
            .name(transitive_ext.table.table_transitive_extension_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Marker trait indicating that a type is an extension of the {table_schema_ref} table.\n\n\
                        This trait is automatically implemented for all tables that directly or \
                        indirectly extend the {table_schema_ref} table. It captures the transitive closure of the extension \
                        relationship, allowing where clauses to be simplified.",
                        table_schema_ref=transitive_ext.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .super_traits(transitive_ext.table.ancestral_extended_tables(transitive_ext.database).into_iter().chain(std::iter::once(transitive_ext.table)).map(|ancestor_table| {
                let ancestor_model_ref = ancestor_table.model_ref(transitive_ext.workspace).expect("Failed to get the table model ref for an ancestral extended table");
                extensin_of.set_generic_field(&generic_type("Extended"), ancestor_model_ref.into()).unwrap().into()
            }))
            .build()
            .expect("Failed to convert internal trait builder into internal trait")
    }
}
