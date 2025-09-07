//! Submodule providing the code to generate the foreign defined completions.
//!
//! A foreign define completion is a read operation which retrieves from the DB
//! informations regarding one or more columns which are defined by a foreign
//! key constraint.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Codegen, Table, errors::WebCodeGenError, table_metadata::PartialBuilderKind, traits::TableLike,
};

impl Codegen<'_> {
    pub(super) fn foreign_defined_completions(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(Vec<TokenStream>, Vec<TokenStream>), WebCodeGenError> {
        let mut foreign_defined_completions = Vec::new();
        let mut extra_requirements = Vec::new();

        let buildable_trait = table.setter_trait_ty()?;
        for foreign_define_column in table.foreign_definer_columns(conn)? {
            let mut foreign_definer_ops = Vec::new();
            let foreign_define_column_ident = foreign_define_column.snake_case_ident()?;
            for (foreign_table, foreign_definer_constraints) in
                foreign_define_column.foreign_definer_constraints_by_table(conn)?
            {
                let foreign_table_primary_key_columns = foreign_table.primary_key_columns(conn)?;
                let foreign_table_struct = foreign_table.import_struct_path()?;
                let foreign_table_snake_case = foreign_table.snake_case_ident()?;
                let mut assignments = Vec::new();
                for foreign_definer_constraint in foreign_definer_constraints {
                    for (local_column, foreign_column) in
                        foreign_definer_constraint.column_mappings(conn)?
                    {
                        if foreign_table_primary_key_columns.contains(&foreign_column) {
                            continue;
                        }
                        let local_column_setter = local_column.getter_ident()?;
                        let foreign_column_ident = foreign_column.snake_case_ident()?;

                        assignments.push(match (local_column.is_nullable(), foreign_column.is_nullable()) {
                            (true, true) | (false, false) => quote! {
                                self = <Self as #buildable_trait>::#local_column_setter(
                                    self,
                                    #foreign_table_snake_case.#foreign_column_ident
                                )?;
                            },
                            (true, false) => quote! {
                                self = <Self as #buildable_trait>::#local_column_setter(
                                    self,
                                    Some(#foreign_table_snake_case.#foreign_column_ident)
                                )?;
                            },
                            (false, true) => quote! {
                                if let Some(#foreign_column_ident) = #foreign_table_snake_case.#foreign_column_ident {
                                    self = <Self as #buildable_trait>::#local_column_setter(
                                        self,
                                        #foreign_column_ident
                                    )?;
                                }
                            }
                        });
                    }
                }

                if assignments.is_empty() {
                    continue;
                }

                extra_requirements.push(quote! {
                    #foreign_table_struct: web_common_traits::database::Read<C>
                });
                foreign_definer_ops.push(quote! {
                    let #foreign_table_snake_case = #foreign_table_struct::read(#foreign_define_column_ident, conn)?;
                    #(#assignments)*
                });
            }

            if foreign_definer_ops.is_empty() {
                continue;
            }

            foreign_defined_completions.push(if let Some((build_kind, _, _)) = foreign_define_column.requires_partial_builder(conn)? {
                assert_eq!(build_kind, PartialBuilderKind::Discretional);
                quote! {
                    if let web_common_traits::database::IdOrBuilder::Id(#foreign_define_column_ident) = self.#foreign_define_column_ident {
                        #(#foreign_definer_ops)*
                    }
                }
            } else {
                quote! {
                    if let Some(#foreign_define_column_ident) = self.#foreign_define_column_ident {
                        #(#foreign_definer_ops)*
                    }
                }
            });
        }

        if !foreign_defined_completions.is_empty() {
            let attributes = table.insertable_enum_ty()?;
            extra_requirements.push(quote! {
                Self: #buildable_trait<Attributes=#attributes>
            });
        }

        Ok((foreign_defined_completions, extra_requirements))
    }
}
