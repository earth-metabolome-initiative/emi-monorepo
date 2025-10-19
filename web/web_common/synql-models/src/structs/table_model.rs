//! Submodule defining the `TableModel` struct.

use synql_core::{
    prelude::Builder,
    structs::{InternalCrate, InternalData, InternalModule, InternalStruct},
};

use crate::traits::{TableModelLike, column_model_like::ColumnModelLike};

/// Struct representing a SynQL table model.
pub struct TableModel<'data, 'table, T: TableModelLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::Database,
}

impl<'data, 'table, T: TableModelLike + ?Sized> TableModel<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::Database,
    ) -> Self {
        Self { table, workspace, database }
    }
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalData<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        let mut builder = InternalData::new()
            .public()
            .name(table_model.table.table_camel_name())
            .expect("Failed to set name")
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
                    .expect("Failed to build struct variant"),
            );

        if let Some(documentation) = table_model.table.table_doc(table_model.database) {
            builder = builder.documentation(documentation).expect("Failed to set documentation");
        }

        builder.build().expect("Failed to build table model")
    }
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalModule<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        InternalModule::new()
            .public()
            .name("model")
            .expect("Failed to set the module name")
            .documentation(format!(
                "Submodule providing the [`{}`] data model for the `{}` table.",
                table_model.table.table_camel_name(),
                table_model.table.table_name()
            ))
            .expect("Failed to set the module documentation")
            .public()
            .data(table_model.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalCrate<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(table_model.table.table_model_crate_name())
            .expect("Failed to set the crate name")
            .documentation(format!(
                "Crate providing the [`{}`] data model for the `{}` table.",
                table_model.table.table_camel_name(),
                table_model.table.table_name()
            ))
            .expect("Failed to set the crate documentation")
            .module(table_model.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}
