//! Submodule implementing the `From` trait to convert a
//! `TableExtensionAttributes` into an `InternalEnum`.

use synql_core::{
    prelude::Builder,
    structs::{Documentation, InternalEnum, InternalVariant},
};

use crate::{
    structs::TableExtensionAttributes,
    traits::{TableAttributesLike, TableExtensionAttributesLike},
};

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalEnum
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        InternalEnum::new()
            .variants(extension_attributes.table.extended_tables(extension_attributes.database).into_iter().map(|extended_table| {
				let attribute_enum = extended_table.attributes_ref(extension_attributes.workspace).expect(&format!(
					"Failed to retrieve the attributes enum for extended table `{}` from table `{}`",
					extended_table.table_name(),
					extension_attributes.table.table_name()
				));
                let extended_schema_crate_ref = extended_table
                    .table_schema_ref(extension_attributes.workspace)
                    .expect("Failed to get table schema crate ref for extension attribute variant");
				let extended_table_camel_ident = extended_table.table_singular_camel_ident();
                InternalVariant::new()
                    .name(extended_table_camel_ident)
                    .doc(
                        Documentation::new()
                            .documentation(format!(
                                "Extension for the extended {} table.",
                                extended_table.table_schema_doc_path()
                            ))
                            .expect("Failed to set documentation for extension attribute variant")
                            .internal_dependency(extended_schema_crate_ref)
                            .unwrap()
                            .build()
                            .expect("Failed to build documentation for extension attribute variant"),
                    )
                    .ty(attribute_enum)
                    .build()
                    .expect("Failed to build extension attribute variant")
            }))
            .expect("Failed to set variants for extension attributes enum")
            .build()
            .unwrap()
    }
}
