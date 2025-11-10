//! Submodule implementing the `From` trait to convert a `TableInsertable` into
//! an `InternalCrate`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableInsertable,
    traits::{INSERTABLE_MODULE_NAME, TableInsertableLike},
};

impl<'table, T> From<TableInsertable<'table, T>> for InternalCrate
where
    T: TableInsertableLike + ?Sized,
    T::DB: InheritableDatabaseLike,
{
    fn from(insertable: TableInsertable<'table, T>) -> Self {
        let schema_crate_ref = insertable
            .table
            .table_schema_ref(insertable.workspace)
            .expect("Failed to get the table schema ref for the table");
        InternalCrate::new()
            .name(insertable.table.table_insertable_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Crate providing the [`{table_insertable_name}`](crate::{INSERTABLE_MODULE_NAME}::{table_insertable_name}) trait for the {} table.",
                        insertable.table.table_schema_doc_path(),
                        table_insertable_name=insertable.table.table_insertable_name(),
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .module(insertable.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
