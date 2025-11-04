//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalCrate`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalCrate},
};

impl<'data, 'table, T: crate::traits::TableBuildableLike + ?Sized>
    From<super::TableBuildable<'data, 'table, T>> for synql_core::structs::InternalCrate<'data>
{
    fn from(
        buildable: super::TableBuildable<'data, 'table, T>,
    ) -> synql_core::structs::InternalCrate<'data> {
        let schema_ref = buildable
            .table
            .table_schema_ref(buildable.workspace)
            .expect("Failed to get table schema ref");

        InternalCrate::new()
            .name(buildable.table.table_buildable_crate_name())
            .unwrap()
            .documentation(Documentation::new().documentation(format!(
				"Crate containing the buildable struct and associated traits for the {} table.",
				buildable.table.table_schema_doc_path()
			)).unwrap().internal_dependency(schema_ref).unwrap().build().unwrap())
            .module(buildable.into())
            .unwrap()
            .build()
            .unwrap()
    }
}
