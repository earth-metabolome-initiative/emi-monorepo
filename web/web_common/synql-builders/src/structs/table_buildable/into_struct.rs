//! Submodule implementing the `From` trait to convert a `TableBuildable` into
//! an `InternalStruct`.

use sql_relations::traits::{TriangularSameAsForeignKeyLike, TriangularSameAsTableLike};
use sql_traits::traits::ForeignKeyLike;
use synql_builders_metadata::traits::TableBuildableMetadataLike;
use synql_core::{
    prelude::Builder,
    structs::{DataVariantRef, Documentation, InternalAttribute, InternalStruct},
    traits::ColumnSynLike,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> From<TableBuildable<'table, T>> for InternalStruct {
    fn from(buildable: TableBuildable<'table, T>) -> Self {
        InternalStruct::new()
            .attributes(
                buildable
                    .table
                    .extended_tables(buildable.database)
                    .into_iter()
                    .map(|t| t.builder_extension_attribute(buildable.workspace)),
            )
            .unwrap()
            .attribute(buildable.table.insertable_attribute(buildable.workspace))
            .unwrap()
            .attributes(buildable.table.triangular_same_as_foreign_keys(buildable.database).map(
                |fk| {
                    let host_column = fk.host_column(buildable.database).unwrap();
                    let referenced_table = fk.referenced_table(buildable.database);
                    let referenced_builder_ref: DataVariantRef =
                        referenced_table.builder_ref(buildable.workspace).unwrap().into();

                    InternalAttribute::new()
                        .private()
                        .documentation(
                            Documentation::new()
                                .documentation(format!(
                                    "Underlying {} triangular builder field to the {} column.",
                                    if fk
                                        .triangular_same_as(buildable.database)
                                        .unwrap()
                                        .is_mandatory()
                                    {
                                        "mandatory"
                                    } else {
                                        "discretionary"
                                    },
                                    host_column.column_schema_doc_path(buildable.database)
                                ))
                                .unwrap()
                                .build()
                                .unwrap(),
                        )
                        .ty(referenced_builder_ref.optional())
                        .name(host_column.column_snake_name())
                        .unwrap()
                        .build()
                        .unwrap()
                },
            ))
            .unwrap()
            .build()
            .unwrap()
    }
}
