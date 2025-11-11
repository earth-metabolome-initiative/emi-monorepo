//! Submodule implementing the `From` trait to convert a `TableValueSettable`
//! into an `InternalCrate`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableValueSettable,
    traits::{TRAIT_MODULE_NAME, TableValueSettableLike},
};

impl<'table, T> From<TableValueSettable<'table, T>> for InternalCrate
where
    T: TableValueSettableLike + ?Sized,
    T::DB: InheritableDatabaseLike,
{
    fn from(table_relation: TableValueSettable<'table, T>) -> Self {
        let schema_crate_ref = table_relation
            .table
            .table_schema_ref(table_relation.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalCrate::new()
            .name(table_relation.table.table_value_settable_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Crate providing the [`{table_value_settable_trait_name}`](crate::{TRAIT_MODULE_NAME}::{table_value_settable_trait_name}) trait for the {} table.",
                        table_relation.table.table_schema_doc_path(),
                        table_value_settable_trait_name=table_relation.table.table_value_settable_trait_name(),
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
