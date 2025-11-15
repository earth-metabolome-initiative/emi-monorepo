//! Submodule defining the `TableInsertableKeySettable` struct for generating
//! settable traits.

use sql_traits::{
    prelude::ColumnLike,
    traits::{DatabaseLike, ForeignKeyLike, TableLike},
};
use synql_attributes::traits::TableAttributesLike;
use synql_core::{
    prelude::Builder,
    structs::{
        Argument, DataVariantRef, Documentation, InternalToken, Method, WhereClause, Workspace,
    },
    traits::ColumnSynLike,
    utils::generic_type,
};
use synql_diesel_schema::traits::{ColumnSchema, TableSchema};
use synql_models::traits::TableModelLike;

use crate::traits::{ColumnInsertableKeySettableLike, TableInsertableKeySettableLike};

mod into_crate;
mod into_module;
mod into_trait;

#[derive(Debug)]
/// Struct representing a SynQL table insertable key settable trait.
pub struct TableInsertableKeySettable<'table, T: TableInsertableKeySettableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> Clone
    for TableInsertableKeySettable<'table, T>
{
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> Copy
    for TableInsertableKeySettable<'table, T>
{
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> TableInsertableKeySettable<'table, T> {
    /// Creates a new `TableInsertableKeySettable` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing
    ///   `TableInsertableKeySettableLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }

    /// Generates the setter methods for all value-settable columns in the
    /// table.
    fn insertable_key_setter_methods(&self) -> impl Iterator<Item = Method> + '_ {
        self.table
            .insertable_key_settable_foreign_keys(self.database)
            .map(|foreign_key| self.insertable_key_setter_method(foreign_key))
    }

    /// Generates the setter method for a column.
    fn insertable_key_setter_method(
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

        let mca_referenced_table_model_ref = mca_referenced_table
            .model_ref(self.workspace)
            .expect("Failed to get the table model ref for the referenced table");

        let extension_of = self
            .workspace
            .external_trait("ExtensionOf")
            .expect("Failed to get ExtensionOf trait")
            .set_generic_field(
                &generic_type("Extended"),
                mca_referenced_table_model_ref.clone().into(),
            )
            .unwrap();

        let identifiable = self
            .workspace
            .external_trait("Identifiable")
            .expect("Failed to get Identifiable trait");

        let argument_type = DataVariantRef::generic(host_column.column_acronym_generic());

        let mut return_type = DataVariantRef::self_type(None);
        let mut error_documentation = None;

        if host_column.has_insertable_key_settable_check_constraints(self.database) {
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
            .where_clauses([
                WhereClause::new()
                    .left(argument_type.clone())
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
                    .left(quote::quote! {for<'a> &'a #argument_type})
                    .right(
                        InternalToken::new()
                            .stream(quote::quote! {
                                #identifiable<Id=<&'a #mca_referenced_table_model_ref as #identifiable>::Id>
                            })
                            .employed_trait(identifiable.into())
                            .build()
                            .expect("Failed to build the right side of the where clause"),
                    )
                    .build()
                    .expect("Failed to build where clause"),
            ])
            .unwrap()
            .error_documentations(error_documentation)
            .return_type(return_type)
            .build()
            .expect("Failed to build the setter method")
    }
}
