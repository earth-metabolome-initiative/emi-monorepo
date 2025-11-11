//! Submodule implementing the `From` trait to convert a `TableValueSettable`
//! into an `InternalTrait`.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalTrait},
};

use crate::{structs::TableValueSettable, traits::TableValueSettableLike};

impl<'table, T> From<TableValueSettable<'table, T>> for InternalTrait
where
    T: TableValueSettableLike + ?Sized,
    T::DB: InheritableDatabaseLike,
{
    fn from(table_settable: TableValueSettable<'table, T>) -> Self {
        let schema_crate_ref = table_settable
            .table
            .table_schema_ref(table_settable.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalTrait::new()
            .public()
            .name(table_settable.table.table_value_settable_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Trait providing methods to set the value values of a {} table.",
                        table_settable.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .build()
                    .unwrap(),
            )
            .methods(table_settable.value_setter_methods())
            .expect("Failed to set the internal trait methods")
            .sized()
            .unwrap()
            .build()
            .expect("Failed to convert internal trait builder into internal trait")
    }
}
