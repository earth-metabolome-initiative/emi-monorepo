//! Submodule implementing the `From` trait to convert a `TableModel` into an
//! `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{structs::TableModel, traits::TableModelLike};

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalModule
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        InternalModule::new()
            .public()
            .name("model")
            .expect("Failed to set the module name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the [`{}`] data model for the `{}` table.",
                        table_model.table.table_singular_camel_name(),
                        table_model.table.table_name()
                    ))
                    .unwrap()
                    .build()
                    .expect("Failed to build documentation"),
            )
            .public()
            .internal_tokens(table_model.extension_of_impls())
            .data(table_model.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}
