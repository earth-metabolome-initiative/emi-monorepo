//! Submodule implementing the `From` trait to convert a `TableRelations`
//! into an `InternalTrait`.

use sql_relations::traits::{HorizontalSameAsForeignKeyLike, VerticalSameAsForeignKeyLike};
use synql_core::{
    prelude::{Builder, DatabaseLike, ForeignKeyLike},
    structs::{Documentation, InternalTrait},
    utils::generic_type,
};

use crate::{structs::TableRelations, traits::TableRelationsLike};

impl<'table, T> From<TableRelations<'table, T>> for InternalTrait
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'table, T>) -> Self {
        let model_ref = table_relation.model_ref();
        let ancestor = table_relation
            .workspace
            .external_trait("Ancestor")
            .expect("Failed to get ExtensionOf trait from workspace")
            .set_generic_field(&generic_type("Extended"), model_ref.clone().into())
            .unwrap();
        let schema_crate_ref = table_relation
            .table
            .table_schema_ref(table_relation.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        InternalTrait::new()
            .public()
            .name(table_relation.table.table_relations_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(Documentation::new()
                .documentation(format!(
                    "Trait providing methods to access the relations of the {} struct for the {} table.",
                    model_ref.documentation_path(),
                    table_relation.table.table_schema_doc_path()
                ))
                .unwrap()
                .internal_dependency(schema_crate_ref)
                .build()
                .unwrap())
            .generic(syn::parse_quote! {C})
            .unwrap()
            .super_trait(
                ancestor.into()
            )
            .methods(
                table_relation
                    .table
                    .foreign_keys(table_relation.database)
                    // We filter out foreign keys that start from the primary key of the host table,
                    // as those should be handled by the `Read` trait implementation.
                    .filter(|fk| !fk.is_host_primary_key(table_relation.database) && !fk.is_vertical_same_as(table_relation.database) && !fk.is_horizontal_same_as(table_relation.database))
                    // Temporarely, we only support foreign keys that reference primary keys.
                    .filter(|fk| fk.is_referenced_primary_key(table_relation.database))
                    .map(|fk: &<T::DB as DatabaseLike>::ForeignKey| {
                        if fk.is_referenced_primary_key(table_relation.database) {
                            table_relation.read_based_method(fk)
                        } else {
                            todo!("Non-primary key referenced foreign keys {fk:?} are not yet supported")
                        }
                    }),
            )
            .expect("Failed to set the internal trait methods")
            .build()
            .expect("Failed to convert internal trait builder into internal trait")
    }
}
