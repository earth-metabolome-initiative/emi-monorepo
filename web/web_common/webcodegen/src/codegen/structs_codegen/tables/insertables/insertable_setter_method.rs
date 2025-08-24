//! Submodule taking care of the definition of the insertable builder struct for
//! a table and the most basic traits.

use std::fmt::Debug;
use crate::traits::TableLike;

use algebra::impls::{CSR2D, SquareCSR2D};
use diesel::PgConnection;
use graph::{
    prelude::GenericGraph,
    traits::{MonopartiteGraph, MonoplexGraph},
};
use proc_macro2::TokenStream;
use quote::quote;
use sorted_vec::prelude::SortedVec;
use syn::Ident;

use crate::{
    Codegen, Column, Table,
    errors::{CheckConstraintError, CodeGenerationError, WebCodeGenError},
};

mod same_as_assignments;

impl Codegen<'_> {
    pub(crate) fn generate_method_check_constraints<C: AsRef<Column> + Debug>(
        &self,
        table: &Table,
        columns: &[C],
        conn: &mut PgConnection,
    ) -> Result<Vec<TokenStream>, WebCodeGenError> {
        let all_columns = table.columns(conn)?;
        let insertable_columns = table.insertable_columns(conn, false)?;
        let nullable_insertable_columns: Vec<Column> = insertable_columns
            .iter()
            .filter(|column| !columns.iter().any(|c| c.as_ref() == *column))
            .map(Column::to_nullable)
            .collect();

        let mut check_constraints = columns
            .iter()
            .map(|column| column.as_ref().check_constraints(conn))
            .collect::<Result<Vec<_>, WebCodeGenError>>()?
            .into_iter()
            .flatten()
            .filter(|constraint| !constraint.is_postgis_constraint())
            .collect::<Vec<_>>();

        check_constraints.sort_unstable();
        check_constraints.dedup();

        check_constraints
            .into_iter()
            .filter_map(|constraint| {
                let outcome = constraint.to_syn(
                    columns,
                    &nullable_insertable_columns,
                    self.check_constraints_extensions.as_slice(),
                    conn,
                );
                if let Err(WebCodeGenError::CodeGenerationError(
                    CodeGenerationError::CheckConstraintError(
                        CheckConstraintError::NoInvolvedColumns(unknown_column, _),
                    ),
                )) = &outcome
                {
                    if all_columns.contains(unknown_column.as_ref())
                        && !insertable_columns.contains(unknown_column.as_ref())
                    {
                        return None;
                    }
                }
                Some(outcome)
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()
    }

    /// Generates the most specific insertable builder type for the provided
    /// extension tree and the root table.
    ///
    /// # Arguments
    ///
    /// * `extension_tree` - The extension tree to generate the generics for.
    pub fn generate_extension_tree_generics(
        &self,
        root_table: &Table,
        extension_tree: &GenericGraph<
            &'_ SortedVec<Table>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        >,
    ) -> Result<(TokenStream, Vec<Table>), WebCodeGenError> {
        let extension_network =
            self.table_extension_network().expect("The extension network should be available");

        let root_table_id = extension_network
            .extension_graph
            .nodes_vocabulary()
            .binary_search(root_table)
            .expect("The root table should be present in the vocabulary");
        let builder_ident = root_table.insertable_builder_ty()?;
        let mut right_generics: Vec<TokenStream> = Vec::new();
        let mut left_generic_tables = Vec::new();

        for ancestor_id in extension_network.extension_graph.successors(root_table_id) {
            let ancestor_table = extension_tree
                .nodes_vocabulary()
                .get(ancestor_id)
                .expect("The ancestor table should be present in the vocabulary");
            right_generics.push(if extension_tree.has_successor(root_table_id, ancestor_id) {
                let (recursive_right_generics, recursive_left_generics) =
                    self.generate_extension_tree_generics(ancestor_table, extension_tree)?;
                left_generic_tables.extend(recursive_left_generics);
                quote! { #recursive_right_generics }
            } else {
                let ancestor_table_ident = ancestor_table.struct_ident()?;
                left_generic_tables.push(ancestor_table.clone());
                quote! { #ancestor_table_ident }
            });
        }

        let maybe_right_generics =
            if !right_generics.is_empty() { Some(quote! { <#(#right_generics),*> }) } else { None };

        Ok((
            quote! {
                #builder_ident #maybe_right_generics
            },
            left_generic_tables,
        ))
    }

    /// Generates the implementation of the setter method for the provided
    /// column for the builder of the provided table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which to generate the setter method.
    /// * `columns` - The columns (potentially foreign) for which to generate
    ///   the setter method.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub(super) fn generate_setter_method<C>(
        &self,
        table: &Table,
        columns: &[C],
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError>
    where
        C: AsRef<Column> + Debug,
    {
        // We assert that the provided column is not part of the primary key,
        // as the primary key setter methods are generated separately.

        let mut column_preprocessing = Vec::new();
        let mut use_requirements = Vec::new();
        let mut where_constraints = Vec::new();

        let setter_method_ident = Column::multi_column_getter_ident(columns)?;
        let insertable_enum = table.insertable_enum_ident()?;
        let mut updated_by_exception = None;
        let mut method_generics = Vec::new();
        let check_constraints: Vec<TokenStream> =
            if columns.iter().any(|column| table.has_column(column.as_ref())) {
                self.generate_method_check_constraints(table, columns, conn)?
            } else {
                Vec::new()
            };

        // If the current table has both `created_by` and `updated_by`, and
        // we are currently assigning the `created_by` column, we need to
        // assign the `updated_by` column as well.
        if columns.iter().any(|column| column.as_ref().is_created_by(conn).unwrap_or(false))
            && !columns.iter().any(|column| column.as_ref().is_updated_by(conn).unwrap_or(false))
            && table.has_updated_by_column(true, conn)?
        {
            updated_by_exception = Some(quote! {
                self = self.updated_by(created_by)?;
            });
        };

        let mut same_as_assignments = Vec::new();
        let mut arguments = Vec::new();
        let mut necessary_columns = Vec::new();
        let mut column_assignments = Vec::new();

        for column in columns.iter() {
            let (this_same_as_assignments, this_necessary_columns) =
                self.generate_same_as_assignments(table, column.as_ref(), conn)?;

            let maybe_mut =
                if this_same_as_assignments.is_empty() { None } else { Some(quote! { mut }) };

            same_as_assignments.extend(this_same_as_assignments);
            necessary_columns.extend(this_necessary_columns);
            necessary_columns.push(column.as_ref().clone());
            let column_camel_case_ident = column.as_ref().camel_case_ident()?;

            let column_snake_case_ident = column.as_ref().snake_case_ident()?;

            // If the column is not part of a foreign key and is a simple column, we want
            // to support the `P` generic and the associated `TryInto` trait.
            let argument_type = if !column.as_ref().has_foreign_keys(conn) {
                let generic_ident = column.as_ref().camel_case_ident()?;
                let column_type = column.as_ref().rust_data_type(conn)?;
                where_constraints.push(quote! {#generic_ident: TryInto<#column_type>});
                where_constraints.push(quote! {<#generic_ident as TryInto<#column_type>>::Error: Into<validation_errors::SingleFieldError>});

                // If the column is local, we convert from the generic `P` to the column type.
                if table.has_column(column.as_ref()) {
                    column_preprocessing.push(quote! {
                        let #column_snake_case_ident = #column_snake_case_ident
                            .try_into()
                            .map_err(|err: <#generic_ident as TryInto<#column_type>>::Error| {
                                Into::into(err)
                                    .rename_field(#insertable_enum::#column_camel_case_ident)
                            })?;
                    });
                }

                method_generics.push(generic_ident.clone());

                quote! {#generic_ident}
            } else if let Some(partial_builder_foreign_key) =
                column.as_ref().requires_partial_builder(conn)?
            {
                let foreign_table = partial_builder_foreign_key
                    .foreign_table(conn)?
                    .expect("The foreign table should be present");
                let foreign_builder_type = foreign_table.insertable_builder_ty()?;
                quote! {#foreign_builder_type}
            } else {
                let rust_type = column.as_ref().rust_data_type(conn)?;
                quote! { #rust_type }
            };

            arguments.push(quote! {
                #maybe_mut #column_snake_case_ident: #argument_type
            });

            let column_path = self.table_extension_network().unwrap().extension_foreign_keys_path(
                table,
                column.as_ref(),
                conn,
            );
            column_assignments.push(if !table.has_column(column.as_ref()) {
                // If the column currently being processed is not part of the table,
                // it means it is from an ancestor table, and therefore we need to
                // discover from which of the ancestors it is. This is
                // particularly relevant for the cases where a table has
                // several ancestors.
                let first_ancestor = column_path
                    .as_ref()
                    .expect("The column path should not be empty")
                    .first()
                    .expect("The column path should have at least one ancestor");
                let first_ancestor_ident = first_ancestor.constraint_ident(conn)?;
                let enum_lambda = self.into_field_name_lambda(table, column.as_ref(), conn)?;

                if columns.len() == 1 {
                    let getter_ident = column.as_ref().getter_ident()?;
                    quote! {
                        self.#first_ancestor_ident = self.#first_ancestor_ident.#getter_ident(
                            #column_snake_case_ident
                        ).map_err(|e| e.into_field_name(#enum_lambda))?;
                    }
                } else if column.as_ref() == columns[0].as_ref() {
                    let getter_ident = Column::multi_column_getter_ident(columns)?;
                    let column_snake_case_idents = columns
                        .iter()
                        .map(|c| c.as_ref().snake_case_ident())
                        .collect::<Result<Vec<_>, WebCodeGenError>>()?;
                    quote! {
                        self.#first_ancestor_ident = self.#first_ancestor_ident.#getter_ident(
                            #(#column_snake_case_idents),*
                        ).map_err(|e| e.into_field_name(#enum_lambda))?;
                    }
                } else {
                    TokenStream::new()
                }
            } else if let Some(partial_builder_foreign_key) = column.as_ref().requires_partial_builder(conn)? {
                let foreign_table = partial_builder_foreign_key
                    .foreign_table(conn)?
                    .expect("The foreign table should be present");
                let foreign_builder_type = foreign_table.insertable_builder_ty()?;
                use_requirements.push(quote::quote! {
                    use web_common_traits::database::ExtendableBuilder;
                });
                let foreign_attributes = foreign_table.insertable_enum_ty()?;
                where_constraints.push(quote! {
                    #foreign_builder_type: web_common_traits::database::ExtendableBuilder<
                        Attributes=#foreign_attributes
                    >
                });
                quote! {
                    self.#column_snake_case_ident = self.#column_snake_case_ident.extend_builder(
                        #column_snake_case_ident
                    ).map_err(|e|
                        e.into_field_name(|attribute| #insertable_enum::#column_camel_case_ident(attribute))
                    )?;
                }
            } else if column.as_ref().is_nullable() {
                quote! {
                    self.#column_snake_case_ident = #column_snake_case_ident;
                }
            } else {
                quote! {
                    self.#column_snake_case_ident = Some(#column_snake_case_ident);
                }
            });
        }

        necessary_columns.sort_unstable();
        necessary_columns.dedup();

        let maybe_where_constraints =
            where_constraints.is_empty().then(|| TokenStream::new()).unwrap_or_else(|| {
                quote! {
                    where #(#where_constraints),*
                }
            });

        // The generics to be inserted are defined by which is the column which is
        // currently being processed. The column may be from the current table
        // or from an ancestor table. The generics need to be defined in such a
        // way to allow for the column to be accessible. The path to the column
        // is determined by the tables extension graph.

        let extension_tree =
            self.table_extension_network().unwrap().extension_tree(table, &necessary_columns);

        // First, we determine the parent builder struct that is defined by the
        // provided extension path, and we add to the left generics the necessary
        // tables, as we use the table struct identifiers as the generics.

        let (insertable_builder_ident, left_generics) =
            self.generate_extension_tree_generics(table, &extension_tree)?;
        let left_generics_idents: Vec<Ident> = left_generics
            .iter()
            .map(|t| t.struct_ident())
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        let maybe_impl_left_generics = if !left_generics.is_empty() {
            Some(quote! { <#(#left_generics_idents),*> })
        } else {
            None
        };

        let documentation_message = format!(
            "Sets the value of the {} {} from table `{}`.",
            columns
                .iter()
                .map(|column| {
                    format!("`{}.{}`", column.as_ref().table_name, column.as_ref().column_name)
                })
                .collect::<Vec<_>>()
                .join(", "),
            if columns.len() > 1 { "columns" } else { "column" },
            table.table_name,
        );

        let maybe_method_generics =
            if method_generics.is_empty() { None } else { Some(quote! {<#(#method_generics),*>}) };

        Ok(quote! {
            impl #maybe_impl_left_generics #insertable_builder_ident {
                #[doc = #documentation_message]
                pub fn #setter_method_ident #maybe_method_generics (
                    mut self, #(#arguments),*
                ) -> Result<Self, web_common_traits::database::InsertError<#insertable_enum>>
                #maybe_where_constraints
                {
                    #(#use_requirements)*
                    #(#column_preprocessing)*
                    #(#check_constraints)*
                    #(#same_as_assignments)*
                    #(#column_assignments)*
                    #updated_by_exception
                    Ok(self)
                }
            }
        })
    }
}
