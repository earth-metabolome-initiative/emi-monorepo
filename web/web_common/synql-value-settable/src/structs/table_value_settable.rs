//! Submodule defining the `TableValueSettable` struct for generating settable
//! traits.

use quote::quote;
use sql_relations::{prelude::ColumnLike, traits::InheritableDatabaseLike};
use sql_traits::traits::DatabaseLike;
use synql_attributes::traits::TableAttributesLike;
use synql_core::{
    prelude::Builder,
    structs::{
        Argument, DataVariantRef, Documentation, InternalToken, Method, WhereClause, Workspace,
    },
    traits::ColumnSynLike,
    utils::generic_type,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::traits::TableValueSettableLike;

mod into_crate;
mod into_module;
mod into_trait;

#[derive(Debug)]
/// Struct representing a SynQL table value settable trait.
pub struct TableValueSettable<'table, T: TableValueSettableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableValueSettableLike + ?Sized> Clone for TableValueSettable<'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableValueSettableLike + ?Sized> Copy for TableValueSettable<'table, T> {}

impl<'table, T: TableValueSettableLike + ?Sized> TableValueSettable<'table, T> {
    /// Creates a new `TableValueSettable` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing `TableValueSettableLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }

    /// Generates the setter methods for all value-settable columns in the
    /// table.
    fn value_setter_methods(&self) -> impl Iterator<Item = Method> + '_
    where
        T::DB: InheritableDatabaseLike,
    {
        self.table
            .value_settable_columns(self.database)
            .map(|column| self.value_setter_method(column))
    }

    /// Generates the setter method for a column.
    fn value_setter_method(&self, column: &'table <T::DB as DatabaseLike>::Column) -> Method {
        let table_schema_ref = self
            .table
            .table_schema_ref(self.workspace)
            .expect("Failed to get the table schema ref for the table relations");

        let column_type =
            column.external_postgres_type(self.workspace, self.database).expect(&format!(
                "Failed to get the external type for column `{}.{}`",
                self.table.table_name(),
                column.column_name()
            ));

        let table_attributes =
            self.table.attributes_ref(self.workspace).expect("Failed to retrieve table attributes");

        let validation_error = self
            .workspace
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
            .expect("Failed to get ValidationError type ref")
            .set_generic_field(&generic_type("FieldName"), table_attributes.into())
            .expect("Failed to set generic field for ValidationError");
        let column_acronym = column.column_acronym_generic();

        Method::new()
            .name(column.column_snake_name())
            .expect("Failed to set the method name")
            .private()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Sets the value of the {} column.",
                        column.column_schema_doc_path(self.database)
                    ))
                    .expect("Failed to set the method documentation")
                    .internal_dependency(table_schema_ref.clone())
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
            .generic(column_acronym.clone())
            .argument(
                Argument::new()
                    .name(column.column_snake_name())
                    .expect("Failed to set the argument name")
                    .arg_type(DataVariantRef::generic(column_acronym.clone()))
                    .documentation(
                        Documentation::new()
                            .documentation(column.column_doc(self.database).expect(&format!(
                                "Failed to get the documentation for column `{}.{}`",
                                self.table.table_name(),
                                column.column_name()
                            )))
                            .expect("Failed to set the argument documentation")
                            .build()
                            .expect("Failed to build the argument documentation"),
                    )
                    .build()
                    .expect("Failed to build the argument"),
            )
            .expect("Failed to add argument to method")
            .where_clauses(
                [
                    WhereClause::new()
                    .left(column_acronym.clone())
                    .right(InternalToken::new().data(column_type.clone().into()).stream(quote! {
                        TryInto<#column_type>
                    }).build().expect("Failed to build the right side of the where clause"))
                    .build()
                    .expect("Failed to build where clause"),
                    WhereClause::new()
                        .left(validation_error.format_with_generics())
                        .right(InternalToken::new().data(column_type.clone().into()).stream(quote! {
                            From<<#column_acronym as TryInto<#column_type>>::Error>
                        }).build().expect("Failed to build the right side of the where clause"))
                        .build()
                        .expect("Failed to build where clause")
                ]
            )
            .unwrap()
            .return_type(DataVariantRef::result(DataVariantRef::self_type(None), validation_error))
            .error_documentation(Documentation::new()
                .documentation(format!(
                    "Returns a [`ValidationError`](validation_errors::prelude::ValidationError) if the provided value for column {} is invalid.",
                    column.column_schema_doc_path(self.database)
                ))
                .expect("Failed to set the error documentation").internal_dependency(table_schema_ref).build().unwrap())
            .build()
            .expect("Failed to build the setter method")
    }
}
