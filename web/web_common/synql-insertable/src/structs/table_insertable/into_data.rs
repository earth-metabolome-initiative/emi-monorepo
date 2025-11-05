//! Submodule implementing the `From` trait to convert a `TableInsertable` into
//! an `InternalData`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalData},
};
use synql_models::traits::ColumnModelLike;

use crate::{structs::TableInsertable, traits::TableInsertableLike};

impl<'data, 'table, T: TableInsertableLike + ?Sized> From<TableInsertable<'data, 'table, T>>
    for InternalData<'data>
{
    fn from(insertable: TableInsertable<'data, 'table, T>) -> Self {
        let table_model_ref = insertable
            .table
            .model_ref(insertable.workspace)
            .expect("Failed to get the table model ref for the insertable data");
        let struct_name = insertable.table.table_insertable_name();
        InternalData::new()
            .public()
            .name(struct_name)
            .expect("Failed to set insertable struct name")
            .derive(insertable.diesel_derives())
            .expect("Failed to add derives to insertable struct")
            .decorator(insertable.table_decorator())
            .expect("Failed to add table decorator to insertable struct")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Insertable struct variant of {}.",
                        table_model_ref.documentation_path()
                    ))
                    .unwrap()
                    .internal_dependency(table_model_ref.crate_ref().unwrap().clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .variant(
                synql_core::structs::InternalStruct::new()
                    .attributes(insertable.table.insertable_columns(insertable.database).map(
                        |column| {
                            column.attribute(insertable.database, insertable.workspace).optional()
                        },
                    ))
                    .expect("Failed to set attributes")
                    .build()
                    .expect("Failed to build struct variant")
                    .into(),
            )
            .build()
            .expect("Failed to build insertable data")
    }
}
