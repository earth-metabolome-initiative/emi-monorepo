//! Submodule defining the `TableBuildableKeySettable` struct for generating
//! settable traits.

use sql_relations::prelude::TriangularSameAsForeignKeyLike;
use sql_traits::{
    prelude::ColumnLike,
    traits::{DatabaseLike, ForeignKeyLike, TableLike},
};
use synql_attributes::traits::TableAttributesLike;
use synql_builders_metadata::prelude::TableBuildableMetadataLike;
use synql_core::{
    prelude::Builder,
    structs::{
        Argument, DataVariantRef, Documentation, ExternalCrate, InternalToken, Method, WhereClause,
        Workspace,
    },
    traits::ColumnSynLike,
    utils::generic_type,
};
use synql_diesel_schema::traits::{ColumnSchema, TableSchema};
use synql_models::traits::TableModelLike;
use synql_transitive_extension::traits::TableTransitiveExtensionLike;

use crate::traits::{ColumnBuildableKeySettableLike, TableBuildableKeySettableLike};

mod into_crate;
mod into_module;
mod into_trait;

#[derive(Debug)]
/// Struct representing a SynQL table buildable key settable trait.
pub struct TableBuildableKeySettable<'table, T: TableBuildableKeySettableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableBuildableKeySettableLike + ?Sized> Clone
    for TableBuildableKeySettable<'table, T>
{
    #[inline]
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableBuildableKeySettableLike + ?Sized> Copy
    for TableBuildableKeySettable<'table, T>
{
}

impl<'table, T: TableBuildableKeySettableLike + ?Sized> TableBuildableKeySettable<'table, T> {
    /// Creates a new `TableBuildableKeySettable` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing
    ///   `TableBuildableKeySettableLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    #[inline]
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }

    /// Generates the setter methods for all value-settable columns in the
    /// table.
    fn buildable_key_setter_methods(&self) -> impl Iterator<Item = Method> + '_ {
        self.table.buildable_key_settable_foreign_keys(self.database).map(|foreign_key| {
            if let Some(triangular) = foreign_key.triangular_same_as(self.database) {
                match triangular.horizontal_same_as() {
                    Some(horizontal_same_as) => {
                        self.mandatory_triangular_method(
                            foreign_key,
                            triangular.hypothenuse_same_as(),
                            horizontal_same_as,
                        )
                    }
                    None => {
                        self.discretional_triangular_method(
                            foreign_key,
                            triangular.hypothenuse_same_as(),
                        )
                    }
                }
            } else {
                self.foreign_key_setter_method(foreign_key)
            }
        })
    }

    fn mandatory_triangular_method(
        &self,
        foreign_key: &'table <T::DB as DatabaseLike>::ForeignKey,
        _hypothenuse_same_as: &'table <T::DB as DatabaseLike>::ForeignKey,
        _horizontal_same_as: &'table <T::DB as DatabaseLike>::ForeignKey,
    ) -> Method {
        let host_column = foreign_key.host_column(self.database).unwrap();
        let host_table = host_column.table(self.database);
        let referenced_table = foreign_key.referenced_table(self.database);

        let host_table_schema_ref = host_table
            .table_schema_ref(self.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        let builder_type = referenced_table.builder_ref(self.workspace).unwrap();

        Method::new()
            .name(host_column.column_snake_name())
            .expect("Failed to set the method name")
            .private()
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Sets the foreign key(s) from the {} column.",
                        host_column.column_schema_doc_path(self.database),
                    ))
                    .expect("Failed to set the method documentation")
                    .internal_dependency(host_table_schema_ref.clone())
                    .build()
                    .expect("Failed to build the method documentation"),
            )
            .argument(
                Argument::new()
                    .name("self")
                    .unwrap()
                    .arg_type(DataVariantRef::self_type(None))
                    .build()
                    .expect("Failed to build self argument"),
            )
            .unwrap()
            .generic(host_column.column_acronym_generic())
            .argument(
                Argument::new()
                    .name(host_column.column_snake_name())
                    .expect("Failed to set the argument name")
                    .arg_type(builder_type)
                    .documentation(
                        Documentation::new()
                            .documentation(host_column.column_doc(self.database).expect(&format!(
                                "Failed to get the documentation for column `{}.{}`",
                                self.table.table_name(),
                                host_column.column_name()
                            )))
                            .expect("Failed to set the argument documentation")
                            .build()
                            .expect("Failed to build the argument documentation"),
                    )
                    .build()
                    .expect("Failed to build the argument"),
            )
            .expect("Failed to add argument to method")
            .return_type(DataVariantRef::self_type(None))
            .build()
            .expect("Failed to build the setter method")
    }

    fn discretional_triangular_method(
        &self,
        foreign_key: &'table <T::DB as DatabaseLike>::ForeignKey,
        _hypothenuse_same_as: &'table <T::DB as DatabaseLike>::ForeignKey,
    ) -> Method {
        let host_column = foreign_key.host_column(self.database).unwrap();
        let host_table = host_column.table(self.database);
        let referenced_table = foreign_key.referenced_table(self.database);

        let host_table_schema_ref = host_table
            .table_schema_ref(self.workspace)
            .expect("Failed to get the table schema ref for the table relations");
        let referenced_table_model_ref = referenced_table
            .model_ref(self.workspace)
            .expect("Failed to get the table model ref for the referenced table");

        let extension_of = self
            .workspace
            .external_trait("ExtensionOf")
            .expect("Failed to get ExtensionOf trait")
            .set_generic_field(&generic_type("Extended"), referenced_table_model_ref.clone().into())
            .unwrap();

        let identifiable = self
            .workspace
            .external_trait("Identifiable")
            .expect("Failed to get Identifiable trait");

        let builder_type = referenced_table.builder_ref(self.workspace).unwrap();
        let generic_type = DataVariantRef::generic(host_column.column_acronym_generic());
        let either =
            ExternalCrate::either_of(Some(builder_type.into()), Some(generic_type.reference(None)));

        Method::new()
            .name(host_column.column_snake_name())
            .expect("Failed to set the method name")
            .private()
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Sets the foreign key(s) from the {} column.",
                        host_column.column_schema_doc_path(self.database),
                    ))
                    .expect("Failed to set the method documentation")
                    .internal_dependency(host_table_schema_ref.clone())
                    .build()
                    .expect("Failed to build the method documentation"),
            )
            .argument(
                Argument::new()
                    .name("self")
                    .unwrap()
                    .arg_type(DataVariantRef::self_type(None))
                    .build()
                    .expect("Failed to build self argument"),
            )
            .unwrap()
            .generic(host_column.column_acronym_generic())
            .argument(
                Argument::new()
                    .name(host_column.column_snake_name())
                    .expect("Failed to set the argument name")
                    .arg_type(either)
                    .documentation(
                        Documentation::new()
                            .documentation(host_column.column_doc(self.database).expect(&format!(
                                "Failed to get the documentation for column `{}.{}`",
                                self.table.table_name(),
                                host_column.column_name()
                            )))
                            .expect("Failed to set the argument documentation")
                            .build()
                            .expect("Failed to build the argument documentation"),
                    )
                    .build()
                    .expect("Failed to build the argument"),
            )
            .expect("Failed to add argument to method")
            .where_clauses([
                WhereClause::new()
                    .left(generic_type.clone())
                    .right(
                        InternalToken::new()
                            .stream(extension_of.format_with_generics())
                            .employed_trait(extension_of.into())
                            .build()
                            .expect("Failed to build the right side of the where clause"),
                    )
                    .build()
                    .expect("Failed to build where clause"),
                WhereClause::new()
                    .left(quote::quote! {for<'a> &'a #generic_type})
                    .right(
                        InternalToken::new()
                            .stream(quote::quote! {
                                #identifiable<Id=<&'a #referenced_table_model_ref as #identifiable>::Id>
                            })
                            .employed_trait(identifiable.into())
                            .build()
                            .expect("Failed to build the right side of the where clause"),
                    )
                    .build()
                    .expect("Failed to build where clause"),
            ])
            .unwrap()
            .return_type(DataVariantRef::self_type(None))
            .build()
            .expect("Failed to build the setter method")
    }

    /// Generates the setter method for a column.
    fn foreign_key_setter_method(
        &self,
        foreign_key: &'table <T::DB as DatabaseLike>::ForeignKey,
    ) -> Method {
        let host_column = foreign_key.host_column(self.database).unwrap();

        let host_table_schema_ref = host_column
            .table(self.database)
            .table_schema_ref(self.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        let referenced_tables = host_column
            .non_composite_foreign_keys(self.database)
            .map(|fk| fk.referenced_table(self.database))
            .collect::<Vec<_>>();

        assert!(
            !referenced_tables.is_empty(),
            "Expected at least one referenced table for foreign key column {}.{}",
            host_column.table(self.database).table_name(),
            host_column.column_name()
        );

        let (referenced_table, other_referenced_tables) = referenced_tables.split_first().unwrap();

        let mca_referenced_table = referenced_table
            .most_recent_common_ancestor(self.database, other_referenced_tables)
            .unwrap();

        let identifiable = self
            .workspace
            .external_trait("Identifiable")
            .expect("Failed to get Identifiable trait");

        let argument_type = DataVariantRef::generic(host_column.column_acronym_generic());

        let mut return_type = DataVariantRef::self_type(None);
        let mut error_documentation = None;

        if host_column.has_buildable_key_settable_check_constraints(self.database) {
            let table_attributes = self
                .table
                .attributes_ref(self.workspace)
                .expect("Failed to retrieve table attributes");

            let validation_error = self
                .workspace
                .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
                .expect("Failed to get ValidationError type ref")
                .set_generic_field(&generic_type("FieldName"), table_attributes.into())
                .expect("Failed to set generic field for ValidationError");
            return_type = DataVariantRef::result(return_type, validation_error);
            error_documentation = Some(Documentation::new()
                .documentation(&format!(
                    "Returns a [`ValidationError`](validation_errors::prelude::ValidationError) if the provided value for column {} is invalid.",
                    host_column.column_schema_doc_path(self.database)
                ))
                .expect("Failed to set the error documentation").internal_dependency(host_table_schema_ref.clone()).build().unwrap());
        }

        let mca_model_ref = mca_referenced_table
            .model_ref(self.workspace)
            .expect("Failed to get the table model ref for the referenced table");

        let transitive_extension_trait_ref =
            mca_referenced_table.transitive_extension_trait_ref(self.workspace).expect(&format!(
                "Failed to get the transitive extension trait ref for the most recent common ancestor table `{}` for current table `{}`",
                mca_referenced_table.table_name(),
                self.table.table_name()
            ));

        Method::new()
            .name(host_column.column_snake_name())
            .expect("Failed to set the method name")
            .private()
            .documentation(
                Documentation::new()
                    .documentation(&format!(
                        "Sets the foreign key(s) from the {} column.",
                        host_column.column_schema_doc_path(self.database),
                    ))
                    .expect("Failed to set the method documentation")
                    .internal_dependency(host_table_schema_ref.clone())
                    .build()
                    .expect("Failed to build the method documentation"),
            )
            .argument(
                Argument::new()
                    .name("self")
                    .unwrap()
                    .arg_type(DataVariantRef::self_type(None))
                    .build()
                    .expect("Failed to build self argument"),
            )
            .unwrap()
            .generic(host_column.column_acronym_generic())
            .argument(
                Argument::new()
                    .name(host_column.column_snake_name())
                    .expect("Failed to set the argument name")
                    .arg_type(argument_type.reference(None))
                    .documentation(
                        Documentation::new()
                            .documentation(host_column.column_doc(self.database).expect(&format!(
                                "Failed to get the documentation for column `{}.{}`",
                                self.table.table_name(),
                                host_column.column_name()
                            )))
                            .expect("Failed to set the argument documentation")
                            .build()
                            .expect("Failed to build the argument documentation"),
                    )
                    .build()
                    .expect("Failed to build the argument"),
            )
            .expect("Failed to add argument to method")
            .where_clause(
                WhereClause::new()
                    .left(argument_type.clone())
                    .right({
                        InternalToken::new()
                            .stream(quote::quote!{#transitive_extension_trait_ref})
                            .employed_trait(transitive_extension_trait_ref)
                            .build()
                            .expect("Failed to build the right side of the where clause with transitive extension trait")                        
                    })
                    .build()
                    .expect("Failed to build where clause"),
            )
            .unwrap()
            .where_clause(
                WhereClause::new()
                    .left(quote::quote! {for<'a> &'a #argument_type})
                    .right(
                        InternalToken::new()
                            .stream(quote::quote! {
                                #identifiable<Id=<&'a #mca_model_ref as #identifiable>::Id>
                            })
                            .employed_trait(identifiable.clone().into())
                            .data(mca_model_ref.clone().into())
                            .build()
                            .expect("Failed to build the right side of the where clause"),
                    )
                    .build()
                    .expect("Failed to build where clause"),
            )
            .unwrap()
            .error_documentations(error_documentation)
            .return_type(return_type)
            .build()
            .expect("Failed to build the setter method")
    }
}
