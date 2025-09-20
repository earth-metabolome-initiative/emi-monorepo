//! Submodule providing the code to generate the implementation of the
//! [`InsertableVariant`] trait for all required tables.

use std::{collections::HashSet, path::Path, sync::Arc};

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    Codegen, Table, errors::WebCodeGenError, table_metadata::PartialBuilderKind, traits::TableLike,
};
mod foreign_defined_completions;

impl Table {
    #[allow(clippy::too_many_lines)]
    /// Returns the implementation of the `TryInsert` trait for the insertable
    /// builder struct.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the implementation is generated.
    /// * `conn` - The database connection to use for querying the database.
    ///
    /// # Errors
    ///
    /// If the provided connection to the database fails.
    ///
    /// # Implementation details
    ///
    /// The `TryInsert` trait has the goal to return the completed insertable
    /// variant of the insertable builder struct, which may entail inserting
    /// some dependant tables in the database.
    ///
    /// All insertable builders for which it is desirable to implement the
    /// `TryInsert` trait have one or more generics, which represent their
    /// immediately extended tables. Depending on their position in the DAG
    /// of table extensions, these generics may have as value either the parent
    /// table builder or an option of the primary key type of the parent
    /// table. A priori, we do not know which of these generics will have the
    /// primary key type value.
    fn generate_insertable_builder_try_insert_implementation(
        &self,
        conn: &mut PgConnection,
    ) -> Result<(TokenStream, Vec<TokenStream>), WebCodeGenError> {
        let insertable_enum = self.insertable_enum_ty()?;
        let mut maybe_mut_self = false;
        let mut additional_use_imports = Vec::new();
        let right_generics = self.generics(conn)?;
        let primary_key_type = self.primary_key_type(conn)?;
        let mut additional_where_requirements = vec![];
        let mut try_insert_generic_constraint: HashSet<Arc<Table>> = HashSet::new();
        let mut attribute_availability_checks = Vec::new();

        let generics_recursive_operation = if right_generics.is_empty() {
            None
        } else {
            additional_use_imports.push(quote! {
                use web_common_traits::database::FromExtension;
            });

            let extension_foreign_keys = self.extension_foreign_keys(conn)?;

            let primary_keys = self.primary_key_columns(conn)?;

            let primary_key = if primary_keys.len() == 1 {
                &primary_keys[0]
            } else {
                unimplemented!(
                    "At this time, we only support DAG tables with a non-composite primary key"
                );
            };
            let primary_key_ident = primary_key.snake_case_ident()?;

            let primary_key_minting = match extension_foreign_keys.len() {
                0 => unreachable!("The table should have at least one extended table"),
                1 => {
                    // In the case of a single generic, it always must be complete
                    // and ready to be inserted or at the very least, in the case
                    // of a `Option<Primary Key Type>`, it must contain the primary key.
                    // As such, we can simply determine the primary key by calling
                    // the `mint_primary_key` method from the `TryInsertGeneric` trait.
                    let extension_foreign_key = &extension_foreign_keys[0];
                    let extension_foreign_key_ident =
                        extension_foreign_key.constraint_ident(conn)?;
                    let foreign_table = extension_foreign_key.foreign_table(conn)?;
                    let foreign_table_generic = foreign_table.struct_ident()?;

                    additional_where_requirements.push(quote! {
                        #foreign_table_generic: web_common_traits::database::TryInsertGeneric<
                            C,
                            PrimaryKey = #primary_key_type
                        >
                    });
                    quote! {
                        self.#extension_foreign_key_ident
                            .mint_primary_key(user_id, conn)
                            .map_err(Self::Error::from_extension)?
                    }
                }
                _ => {
                    // When the table extends several parent tables, we need to insert the primary
                    // parent builders first to obtain the primary key, which we then use to
                    // complete the secondary parent builders. Issue being,
                    // apriori, we do not know which of the generics will be
                    // representing the primary and secondary parent builders,
                    // and therefore unfortunately we need to use an if-else chain.

                    for extension_foreign_key in &extension_foreign_keys {
                        let foreign_table = extension_foreign_key.foreign_table(conn)?;
                        let foreign_table_generic = foreign_table.struct_ident()?;
                        additional_where_requirements.push(quote! {
                            #foreign_table_generic: web_common_traits::database::TryInsertGeneric<
                                C,
                                PrimaryKey = #primary_key_type
                            >
                        });
                    }

                    let chained_if_else = extension_foreign_keys
                        .iter()
                        .map(|extension_foreign_key| {
                            let is_last = extension_foreign_key == extension_foreign_keys.last().unwrap();
                            let is_first = extension_foreign_key == extension_foreign_keys.first().unwrap();
                            let extension_foreign_key_ident = extension_foreign_key.constraint_ident(conn)?;

                            let other_foreign_keys_handling = extension_foreign_keys.iter().filter(|other_extension_foreign_key| {
                                other_extension_foreign_key != &extension_foreign_key
                            })
                            .map(|other_extension_foreign_key| {
                                let other_extension_foreign_key_ident = other_extension_foreign_key.constraint_ident(conn)?;
                                Ok(quote! {
                                    let _ = self.#other_extension_foreign_key_ident
                                        .set_primary_key(#primary_key_ident)
                                        .mint_primary_key(user_id, conn)
                                        .map_err(Self::Error::from_extension)?;
                                })
                            }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

                            let check_statement = if is_last {
                                quote! {
                                    else
                                }
                            } else if is_first {
                                quote! {
                                    if self.#extension_foreign_key_ident.is_complete()
                                }
                            } else {
                                quote! {
                                    else if self.#extension_foreign_key_ident.is_complete()
                                }
                            };

                            Ok(quote! {
                                // If this parent builder is complete, we can mint the primary key.
                                #check_statement {
                                    // We call the `mint_primary_key` method from the `TryInsertGeneric` trait
                                    // which in the case of a `Option<Primary Key Type>` just returns the primary key,
                                    // and in the case of a parent builder, it inserts it into the database and returns
                                    // the primary key.
                                    let #primary_key_ident = self.#extension_foreign_key_ident.mint_primary_key(user_id, conn)
                                        .map_err(Self::Error::from_extension)?;
                                    #(#other_foreign_keys_handling)*

                                    // Finally, we return the primary key.
                                    #primary_key_ident
                                }
                            })
                        })
                        .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
                    quote! {
                        #(#chained_if_else)*
                    }
                }
            };

            Some(quote! {
                let #primary_key_ident = #primary_key_minting;
            })
        };

        let assignments = self
            .insertable_columns(conn, true)?
            .iter()
            .map(|column| {
                let column_ident = column.snake_case_ident()?;
                Ok(if column.is_nullable() {
                    quote! {
                        #column_ident: self.#column_ident
                    }
                } else {
                    if column.is_part_of_extension_primary_key(conn)?.is_none()
                        && column.requires_partial_builder(conn)?.is_none()
                    {
                        let camel_cased_column_ident = column.camel_case_ident()?;
                        attribute_availability_checks.push(quote! {
                        let #column_ident = self.#column_ident.ok_or(
                            common_traits::prelude::BuilderError::IncompleteBuild(
                                #insertable_enum::#camel_cased_column_ident
                            ))?;
                        });
                    }
                    quote! {
                        #column_ident
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        // Next, we need to set the missing attributes for the dependant tables which
        // require the newly minted primary key. First, we need to determine which of
        // these tables are dependant on the primary key of the current table,
        // information which is stored in the `column_same_as` network.
        //
        // Such cases are like the following example: consider a table `A`, with a
        // column `id`, which appears as the target of a foreign key associated
        // to a column `a_id` in table `B`. Next, consider a table `C`, which
        // extends table `A` and additionally has a column `b_id`.
        // When we insert a new row in table `C`, we first insert the parent extended
        // row in table `A`, but up until that time we cannot complete the
        // `a_id` column in table `B`, and in turn we cannot complete the `b_id`
        // column in table `C`. Therefore, we need to insert the row `A` first,
        // next we complete and insert the row `B`, and finally we can complete and
        // insert the row `C`.
        //
        // ASCII illustration of the relationship:
        //
        //     Table A           Table B           Table C
        //   ┌─────────┐       ┌─────────┐       ┌─────────┐
        //   │ id (PK) │◄──────┤ a_id    │◄──────┤ b_id    │
        //   └─────────┘       │ ...     │       │ id (PK) │
        //       ▲             └─────────┘       │ (ext A) │
        //       │                               └─────────┘
        //       │                                     │
        //       │              Extension relationship │
        //       │              (1:1, same value)      │
        //       └─────────────────────────────────────┘
        //
        // Insertion order: A → B → C
        // Dependencies: C.b_id → B.id, B.a_id → A.id, C.id = A.id
        //

        let mut dependant_tables_completion: Vec<TokenStream> = Vec::new();
        let primary_key_columns = self.primary_key_columns(conn)?;

        // Then, for each column in the primary key, we determine the same-as
        // relationships and complete the dependant tables. It is possible that
        // multiple primary key columns have the same local same-as column attribute,
        // in which case it is needful to group by the foreign keys by the local same-as
        // column.
        for (
            partial_builder_column,
            partial_builder_kind,
            potential_same_as_constraint,
            constraint,
        ) in self.partial_builder_columns(conn)?
        {
            let partial_builder_table = constraint.foreign_table(conn)?;
            let partial_builder_table_ref = partial_builder_table.as_ref();
            let foreign_builder = partial_builder_table.insertable_builder_ty()?;
            let partial_builder_table_trait = partial_builder_table.setter_trait_ty()?;

            let local_column_ident = partial_builder_column.snake_case_ident()?;
            let camel_cased_column_ident = partial_builder_column.camel_case_ident()?;
            let builder_ident = match partial_builder_kind {
                PartialBuilderKind::Mandatory => quote! {self.#local_column_ident},
                PartialBuilderKind::Discretional => quote! {#local_column_ident},
            };
            let mut missing_same_as_assignments = primary_key_columns
                .iter()
                .map(|primary_key_column| {
                    let column_ident = primary_key_column.snake_case_ident()?;
                    Ok(primary_key_column
                        .associated_same_as_columns(true, conn)?
                        .into_iter()
                        .filter_map(|(remote_column, _associated_same_as_constraint)| {
                            let remote_table = remote_column.table(conn).ok()?;
                            if remote_table.as_ref() != partial_builder_table_ref {
                                return None;
                            }
                            let remote_column_setter = remote_column.snake_case_ident().ok()?;
                            Some(quote!{
                                #builder_ident = <#foreign_builder as #partial_builder_table_trait>::#remote_column_setter(
                                    #builder_ident,
                                    #column_ident
                                ).map_err(|err| {
                                    err.into_field_name(#insertable_enum::#camel_cased_column_ident)
                                })?;
                            })
                        })
                        .collect())
                })
                .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?
                .into_iter()
                .filter(|ts| !ts.is_empty())
                .collect::<Vec<TokenStream>>();

            if partial_builder_kind.is_discretional() {
                for (primary_key_column, remote_column) in primary_key_columns
                    .iter()
                    .zip(potential_same_as_constraint.columns(conn)?.iter())
                {
                    let column_ident = primary_key_column.snake_case_ident()?;
                    let remote_column_setter = remote_column.snake_case_ident()?;
                    missing_same_as_assignments.push(quote!{
                        #builder_ident = <#foreign_builder as #partial_builder_table_trait>::#remote_column_setter(
                            #builder_ident,
                            #column_ident
                        ).map_err(|err| {
                            err.into_field_name(#insertable_enum::#camel_cased_column_ident)
                        })?;
                    });
                }
            } else if !missing_same_as_assignments.is_empty() {
                maybe_mut_self = true;
            }

            if !try_insert_generic_constraint.contains(&partial_builder_table) {
                additional_where_requirements.push(quote! {
                    #foreign_builder: web_common_traits::database::DispatchableInsertableVariant<
                        C,
                    >
                });
                try_insert_generic_constraint.insert(partial_builder_table);
            }

            let maybe_mut_builder = if missing_same_as_assignments.is_empty() {
                quote! {}
            } else {
                quote! {mut}
            };

            dependant_tables_completion.push(match partial_builder_kind {
                PartialBuilderKind::Mandatory => {
                    quote! {
                        #(#missing_same_as_assignments)*
                        let #local_column_ident = self.#local_column_ident
                            .mint_primary_key(user_id, conn)
                            .map_err(|err| {
                                err.into_field_name(#insertable_enum::#camel_cased_column_ident)
                            })?;
                    }
                }
                PartialBuilderKind::Discretional => {
                    quote! {
                        let #local_column_ident = match self.#local_column_ident {
                            web_common_traits::database::IdOrBuilder::Id(id) => {
                                id
                            },
                            web_common_traits::database::IdOrBuilder::Builder(#maybe_mut_builder #local_column_ident) => {
                                #(#missing_same_as_assignments)*
                                #local_column_ident
                                    .mint_primary_key(user_id, conn)
                                    .map_err(|err| {
                                        err.into_field_name(#insertable_enum::#camel_cased_column_ident)
                                    })?
                            }
                        };
                    }
                }
            });
        }

        if !dependant_tables_completion.is_empty() {
            additional_use_imports.push(quote! {
                use web_common_traits::database::TryInsertGeneric;
            });
        }

        let user_id_ident = if right_generics.is_empty() {
            quote! {_user_id}
        } else {
            quote! {user_id}
        };

        let (foreign_defined_completions, extra_requirements) =
            self.foreign_defined_completions(conn)?;
        additional_where_requirements.extend(extra_requirements);

        if !foreign_defined_completions.is_empty() {
            additional_use_imports.push(quote! {
                use web_common_traits::database::Read;
            });
            maybe_mut_self = true;
        }

        let maybe_mut_self = if maybe_mut_self {
            quote! {mut}
        } else {
            quote! {}
        };

        let conn_ident = if right_generics.is_empty() && foreign_defined_completions.is_empty() {
            quote! {_conn}
        } else {
            quote! {conn}
        };

        Ok((
            quote! {
                fn try_insert(
                    #maybe_mut_self self,
                    #user_id_ident: i32,
                    #conn_ident: &mut C
                ) -> Result<
                    Self::InsertableVariant,
                    Self::Error
                >
                {
                    #(#additional_use_imports)*
                    #(#foreign_defined_completions)*
                    #(#attribute_availability_checks)*
                    #generics_recursive_operation
                    #(#dependant_tables_completion)*
                    Ok(Self::InsertableVariant {
                        #(#assignments),*
                    })
                }
            },
            additional_where_requirements,
        ))
    }
}

impl Codegen<'_> {
    #[allow(clippy::too_many_lines)]
    /// Generates the [`InsertableVariant`] trait implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_variant_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut ifvb_main_module = TokenStream::new();

        for table in tables {
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_struct = table.insertable_variant_ty()?;
            let insertable_builder = table.insertable_builder_ty()?;

            // We build a check to see whether the user is authorized to update
            // the parent tables.

            let mut parent_checks = Vec::new();
            let mut parent_check_where_constraints = Vec::new();

            for parent_key in table.parent_keys(conn)? {
                let parent_table = parent_key.foreign_table(conn)?;

                if !parent_table.allows_updatable(conn)? {
                    continue;
                }

                let parent_key_method = parent_key.constraint_ident(conn)?;
                let parent_key_ident = parent_table.snake_case_ident()?;
                let parent_table_path = parent_table.import_struct_path()?;

                parent_checks.push(if parent_key.is_nullable(conn)? {
                    quote::quote! {
                        if let Some(#parent_key_ident) = insertable_struct.#parent_key_method(conn)? && !#parent_key_ident.can_update(user_id, conn)? {
                            return Err(generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into());
                        }
                    }
                } else {
                    quote::quote! {
                        if !insertable_struct.#parent_key_method(conn)?.can_update(user_id, conn)? {
                            return Err(generic_backend_request_errors::GenericBackendRequestError::Unauthorized.into());
                        }
                    }
                });

                parent_check_where_constraints.push(quote::quote! {
                    #parent_table_path: web_common_traits::database::Updatable<C> + web_common_traits::database::Read<C>
                });
            }

            let maybe_parent_check_use = if parent_checks.is_empty() {
                None
            } else {
                Some(quote::quote! {
                    use web_common_traits::database::Updatable;
                })
            };

            let attributes_enum = table.insertable_enum_ty()?;

            let (try_insert, try_insert_additional_where_clause) =
                table.generate_insertable_builder_try_insert_implementation(conn)?;

            let extension_tables = table.extension_tables(conn)?;
            let generics = table
                .extension_tables(conn)?
                .iter()
                .map(TableLike::struct_ident)
                .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;
            let maybe_generics =
                if generics.is_empty() { None } else { Some(quote! {<#(#generics),*>}) };

            let mut maybe_mut: Option<TokenStream> = None;
            let (
                maybe_complete_most_concrete_table_use,
                maybe_complete_most_concrete_table_constraint,
                maybe_complete_most_concrete_table,
            ) = if table.has_most_concrete_table_column(true, conn)? {
                let current_table_name = table.table_name.clone();
                maybe_mut = Some(quote! {mut});
                (
                    Some(quote! {
                        use web_common_traits::database::MostConcreteTable;
                    }),
                    Some(quote! {
                        Self: web_common_traits::database::MostConcreteTable
                    }),
                    Some(quote! {
                        self.set_most_concrete_table(#current_table_name);
                    }),
                )
            } else {
                (None, None, None)
            };

            let maybe_error_from_extension = if extension_tables.is_empty() {
                None
            } else {
                let from_extensions = extension_tables
                    .iter()
                    .map(|extension_table| {
                        let struct_ident = extension_table.struct_ident()?;
                        Ok(quote! {
                            web_common_traits::database::FromExtension<<#struct_ident as web_common_traits::database::TryInsertGeneric<C>>::Error>
                        })
                    })
                    .collect::<Result<Vec<_>, WebCodeGenError>>()?;
                Some(quote! {
                    Self::Error: #(#from_extensions)+*,
                })
            };

            let maybe_backend_insert = if parent_checks.is_empty() {
                Some(quote! {
                    impl<#(#generics),*> web_common_traits::database::BackendInsertableVariant for #insertable_builder #maybe_generics
                    where
                        Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
                    {}
                })
            } else {
                None
            };

            std::fs::write(&table_file, self.beautify_code(&quote::quote!{

                impl #maybe_generics web_common_traits::database::DispatchableInsertVariantMetadata for #insertable_builder #maybe_generics
                {
                    type Row = #table_path;
                    type Error = web_common_traits::database::InsertError<#attributes_enum>;
                }

				impl #maybe_generics web_common_traits::database::InsertableVariantMetadata for #insertable_builder #maybe_generics
                {
                    type InsertableVariant = #insertable_struct;
                }

                #[cfg(feature = "backend")]
				#maybe_backend_insert

                impl<C: diesel::connection::LoadConnection, #(#generics),*> web_common_traits::database::DispatchableInsertableVariant<C> for #insertable_builder #maybe_generics
                where
                    diesel::query_builder::InsertStatement<
                        <#table_path as diesel::associations::HasTable>::Table,
                        <#insertable_struct as diesel::Insertable<<#table_path as diesel::associations::HasTable>::Table>>::Values,
                    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, #table_path>,
                    Self: web_common_traits::database::InsertableVariant<
                        C,
                        InsertableVariant = #insertable_struct,
                        Row = #table_path,
                        Error = web_common_traits::database::InsertError<#attributes_enum>,
                    >,
                    #(#parent_check_where_constraints,)*
                    #maybe_complete_most_concrete_table_constraint
                {
                    fn insert(
                        #maybe_mut self,
                        user_id: i32,
                        conn: &mut C,
                    ) -> Result<Self::Row, Self::Error> {
                        use diesel::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use web_common_traits::database::InsertableVariant;
                        #maybe_parent_check_use
                        #maybe_complete_most_concrete_table_use

                        #maybe_complete_most_concrete_table
                        let insertable_struct: #insertable_struct = self.try_insert(user_id, conn)?;

                        #(#parent_checks)*

                        Ok(diesel::insert_into(Self::table())
                            .values(insertable_struct)
                            .get_result(conn)?)
                    }
                }
                impl<C: diesel::connection::LoadConnection, #(#generics),*> web_common_traits::database::InsertableVariant<C> for #insertable_builder #maybe_generics
                where
                    diesel::query_builder::InsertStatement<
                        <#table_path as diesel::associations::HasTable>::Table,
                        <#insertable_struct as diesel::Insertable<<#table_path as diesel::associations::HasTable>::Table>>::Values,
                    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, #table_path>,
                    #maybe_error_from_extension
                    #(#try_insert_additional_where_clause),*
                {
                    #try_insert
				}
			}))?;

            ifvb_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&ifvb_main_module))?;

        Ok(())
    }
}
