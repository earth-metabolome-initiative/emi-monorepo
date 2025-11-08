//! Submodule implementing the `From` trait to convert a `TableModel`
//! into an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{
    structs::TableModel,
    traits::{MODEL_MODULE_NAME, TableModelLike},
};

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalCrate
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        let schema_crate_ref = table_model
            .table
            .table_schema_ref(table_model.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalCrate::new()
            .name(table_model.table.table_model_crate_name())
            .expect("Failed to set the crate name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Crate providing the [`{table_singular_camel_name}`](crate::{MODEL_MODULE_NAME}::{table_singular_camel_name}) data model for the {} table.",
                        table_model.table.table_schema_doc_path(),
                        table_singular_camel_name=table_model.table.table_singular_camel_name(),
                    ))
                    .unwrap()
					.internal_dependency(schema_crate_ref)
					.unwrap()
                    .build()
                    .unwrap(),
            )
            .module(table_model.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
