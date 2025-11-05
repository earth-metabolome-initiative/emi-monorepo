//! Submodule implementing the `From` trait to convert a `TableValueSettable`
//! into an `InternalModule`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{
    structs::TableValueSettable,
    traits::{TRAIT_MODULE_NAME, TableValueSettableLike},
};

impl<'data, 'table, T> From<TableValueSettable<'data, 'table, T>> for InternalModule<'data>
where
    T: TableValueSettableLike + ?Sized,
    T::DB: InheritableDatabaseLike,
{
    fn from(table_relation: TableValueSettable<'data, 'table, T>) -> Self {
        let schema_crate_ref = table_relation
            .table
            .table_schema_ref(table_relation.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        InternalModule::new()
            .public()
            .name(TRAIT_MODULE_NAME)
            .expect("Failed to set the module name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] trait for the {} table.",
                        table_relation.table.table_value_settable_trait_name(),
                        table_relation.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .public()
            .internal_trait(table_relation.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}
