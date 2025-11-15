//! Submodule implementing the `From` trait to convert a `SchemaMacro` into
//! an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

use crate::{structs::SchemaMacro, traits::TableSchema};

impl<'table, T> From<SchemaMacro<'table, T>> for InternalCrate
where
    T: synql_core::traits::TableSynLike,
{
    fn from(schema_macro: SchemaMacro<'table, T>) -> Self {
        InternalCrate::new()
            .name(&schema_macro.table.table_schema_crate_name())
            .expect("Invalid crate name")
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Diesel schema crate for the `{}` table.",
                        schema_macro.table.table_name()
                    ))
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .module(schema_macro.into())
            .expect("Invalid module")
            .build()
            .unwrap()
    }
}
