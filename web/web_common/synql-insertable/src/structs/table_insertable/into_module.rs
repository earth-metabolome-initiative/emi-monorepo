//! Submodule implementing the `From` trait to convert a `TableInsertable` into
//! an `InternalModule`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{structs::TableInsertable, traits::TableInsertableLike};

impl<'data, 'table, T: TableInsertableLike + ?Sized> From<TableInsertable<'data, 'table, T>>
    for InternalModule
where
    T::DB: InheritableDatabaseLike,
{
    fn from(value: TableInsertable<'data, 'table, T>) -> Self {
        let module_name = crate::traits::table_insertable_like::INSERTABLE_MODULE_NAME;
        let schema_crate_ref = value
            .table
            .table_schema_ref(value.workspace)
            .expect("Failed to get the table schema ref for the insertable module");

        InternalModule::new()
            .name(module_name)
            .expect("Failed to set insertable module name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] insertable struct for the {} table.",
                        value.table.table_insertable_name(),
                        value.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .data(value.into())
            .expect("Failed to add insertable struct to insertable module")
            .internal_token(value.value_settable_impl())
            .build()
            .expect("Failed to build insertable module")
    }
}
