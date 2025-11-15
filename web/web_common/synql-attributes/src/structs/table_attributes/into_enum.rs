//! Submodule implementing the `From` trait to convert a `TableAttributes` into
//! an `InternalEnum`.

use syn::Ident;
use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalEnum, InternalVariant},
    traits::ColumnSynLike,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::{
    structs::TableAttributes,
    traits::{
        TableAttributesLike, TableExtensionAttributesLike,
        table_extension_attributes_like::EXTENSION_ATTRIBUTES_ENUM_NAME,
    },
};

impl<'table, T: TableAttributesLike + TableExtensionAttributesLike + ?Sized>
    From<TableAttributes<'table, T>> for InternalEnum
{
    fn from(attributes: TableAttributes<'table, T>) -> Self {
        let maybe_extension_variant = if let Some(extension_attributes) =
            attributes.table.extension_attributes_enum_ref(attributes.workspace)
        {
            let extension_ident =
                Ident::new(EXTENSION_ATTRIBUTES_ENUM_NAME, proc_macro2::Span::call_site());
            Some(
                InternalVariant::new()
                    .name(extension_ident)
                    .doc(
                        Documentation::new()
                            .documentation("Extension attributes variant.")
                            .expect("Failed to set documentation for extension attribute variant")
                            .build()
                            .expect(
                                "Failed to build documentation for extension attribute variant",
                            ),
                    )
                    .ty(extension_attributes)
                    .build()
                    .expect("Failed to build extension attribute variant"),
            )
        } else {
            None
        };

        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");

        InternalEnum::new()
            .variants(attributes.table.columns(attributes.database).map(|column| {
                InternalVariant::new()
                    .name(column.column_camel_ident())
                    .doc(
                        Documentation::new()
                            .documentation(&format!(
                                "Attribute representing the {} column.",
                                column.column_schema_doc_path(attributes.database)
                            ))
                            .expect("Failed to set documentation for attribute variant")
                            .internal_dependency(schema_crate_ref.clone())
                            .build()
                            .expect("Failed to build documentation for attribute variant"),
                    )
                    .build()
                    .expect("Failed to build attribute variant")
            }))
            .expect("Failed to set variants for attributes enum")
            .variants(maybe_extension_variant)
            .expect("Failed to set variants for attributes enum")
            .build()
            .expect("Failed to build attributes enum")
    }
}
