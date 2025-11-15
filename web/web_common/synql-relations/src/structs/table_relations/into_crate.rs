//! Submodule implementing the `From` trait to convert a `TableRelations`
//! into an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableRelations,
    traits::{TRAIT_MODULE_NAME, TableRelationsLike},
};

impl<'table, T> From<TableRelations<'table, T>> for InternalCrate
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'table, T>) -> Self {
        let schema_crate_ref = table_relation
            .table
            .table_schema_ref(table_relation.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalCrate::new()
            .name(&table_relation.table.table_relations_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Crate providing the [`{table_relations_trait_name}`](crate::{TRAIT_MODULE_NAME}::{table_relations_trait_name}) trait for the {} table.",
                        table_relation.table.table_schema_doc_path(),
                        table_relations_trait_name=table_relation.table.table_relations_trait_name(),
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .module(table_relation.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
