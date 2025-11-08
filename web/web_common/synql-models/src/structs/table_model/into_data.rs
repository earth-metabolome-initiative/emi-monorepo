//! Submodule implementing the `From` trait to convert a `TableValueSettable`
//! into an `InternalData`.

use quote::quote;
use synql_core::{
    prelude::Builder,
    structs::{Decorator, Documentation, InternalData, InternalStruct, InternalToken},
};

use crate::{
    structs::TableModel,
    traits::{TableModelLike, column_model_like::ColumnModelLike},
};

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalData
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        let table_schema = table_model
            .table
            .schema_module(table_model.workspace)
            .expect("Failed to get the table schema module");
        let snake_case_ident = table_model.table.table_snake_ident();
        InternalData::new()
            .public()
            .name(table_model.table.table_singular_camel_name())
            .expect("Failed to set name")
            .derive(table_model.diesel_derives())
            .expect("Failed to add derives")
            .decorator(
                Decorator::new()
                    .token(
                        InternalToken::new()
                            .private()
                            .stream(quote! {diesel(table_name = #table_schema::#snake_case_ident)})
                            .internal_module(table_schema)
                            .build()
                            .unwrap(),
                    )
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .decorators(table_model.belongs_to_decorators())
            .unwrap()
            .decorators(table_model.primary_key_decorator())
            .unwrap()
            .documentation(
                Documentation::new()
                    .documentation(table_model.table.table_doc(table_model.database).expect(
                        &format!(
                            "Failed to get documentation for table {}",
                            table_model.table.table_name(),
                        ),
                    ))
                    .unwrap()
                    .build()
                    .expect("Failed to build documentation"),
            )
            .variant(
                InternalStruct::new()
                    .attributes(
                        table_model
                            .table
                            .columns(table_model.database)
                            .map(|col| col.attribute(table_model.database, table_model.workspace)),
                    )
                    .expect("Failed to set attributes")
                    .build()
                    .expect("Failed to build struct variant")
                    .into(),
            )
            .build()
            .expect("Failed to build table model")
    }
}
