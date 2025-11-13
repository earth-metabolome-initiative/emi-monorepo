//! Submodule implementing the `From` trait to convert a `TableInsertable` into
//! an `InternalModule`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableBuildable,
    traits::{BUILDABLE_MODULE_NAME, TableBuildableLike},
};

impl<'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'table, T>> for InternalModule
where
    T::DB: InheritableDatabaseLike,
{
    fn from(value: TableBuildable<'table, T>) -> Self {
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the buildable module");

        InternalModule::new()
            .name(BUILDABLE_MODULE_NAME)
            .expect("Failed to set buildable module name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] buildable struct for the {} table.",
                        value.table.table_buildable_name(),
                        value.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .data(value.into())
            .expect("Failed to add buildable struct to buildable module")
            .internal_token(value.value_settable_impl())
            .internal_tokens(value.ancestor_value_settable_impls())
            .internal_token(value.key_settable_impl())
            .internal_tokens(value.ancestor_key_settable_impls())
            .build()
            .expect("Failed to build buildable module")
    }
}
