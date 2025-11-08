//! Submodule implementing the `From` trait to convert a `SchemaMacro` into
//! an `InternalModule`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalModule},
};

use crate::{structs::SchemaMacro, traits::TABLE_SCHEMA_MODULE_NAME};

impl<'table, 'data, T> From<SchemaMacro<'data, 'table, T>> for InternalModule
where
    T: synql_core::traits::TableSynLike,
{
    fn from(schema_macro: SchemaMacro<'data, 'table, T>) -> Self {
        InternalModule::new()
            .name(TABLE_SCHEMA_MODULE_NAME)
            .expect("Invalid name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Diesel schema for the `{}` table.",
                        schema_macro.table.table_name()
                    ))
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .internal_token(schema_macro.into())
            .build()
            .unwrap()
    }
}
