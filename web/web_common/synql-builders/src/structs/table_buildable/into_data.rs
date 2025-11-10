//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalData`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalData},
};

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'table, T>> for InternalData {
    fn from(buildable: TableBuildable<'table, T>) -> Self {
        let table_model_ref = buildable
            .table
            .model_ref(buildable.workspace)
            .expect("Failed to get the table model ref for the buildable data");
        InternalData::new()
            .public()
            .name(buildable.table.table_buildable_name())
            .expect("Failed to set buildable struct name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Buildable struct variant of {}.",
                        table_model_ref.documentation_path()
                    ))
                    .unwrap()
                    .internal_dependency(table_model_ref.crate_ref().unwrap().clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .generics(
                buildable
                    .table
                    .extended_tables(buildable.database)
                    .into_iter()
                    .map(|t| t.table_singular_camel_ident()),
            )
            .unwrap()
            .variant(buildable.into())
            .build()
            .expect("Failed to build insertable data")
    }
}
