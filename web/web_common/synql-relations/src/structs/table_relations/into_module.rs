//! Submodule implementing the `From` trait to convert a `TableRelations`
//! into an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule, InternalTrait},
};

use crate::{
    structs::TableRelations,
    traits::{TRAIT_MODULE_NAME, TableRelationsLike},
};

impl<'data, 'table, T> From<TableRelations<'data, 'table, T>> for InternalModule<'data>
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'data, 'table, T>) -> Self {
        let model_ref = table_relation
            .table
            .model_ref(table_relation.workspace)
            .expect("Failed to get the model ref for the table relations");

        let internal_trait: InternalTrait<'data> = InternalTrait::from(table_relation);
        let auto_blanket = internal_trait.auto_blanket().expect("Failed to generate auto blanket");
        let schema_crate_ref = table_relation
            .table
            .table_schema_ref(table_relation.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        InternalModule::new()
            .public()
            .name(TRAIT_MODULE_NAME)
            .expect("Failed to set the module name")
            .documentation(Documentation::new().documentation(format!(
                "Submodule providing the [`{}`] trait for the [`{}`]({model_ref}) struct and the {} table.",
                table_relation.table.table_relations_trait_name(),
                model_ref.data().name(),
                table_relation.table.table_schema_doc_path()
            )).unwrap().internal_dependency(schema_crate_ref).unwrap().build().unwrap())
            .public()
            .internal_trait(internal_trait)
            .expect("Failed to add the internal data to module")
            .internal_token(auto_blanket)
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}
