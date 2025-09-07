//! Submodule implementing the generation of the `ProcedureTemplate` trait for
//! all procedure templates.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::{Procedure, ProcedureTemplate};

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `ProcedureTemplate` trait for all
    /// procedure templates.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path where to output the generated code.
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_template_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        let mut submodules = Vec::new();
        let procedure_template_table = ProcedureTemplate::root(table_catalog, conn)?;
        let procedure_template_ident = procedure_template_table.struct_ident()?;
        let procedure_table = Procedure::root(table_catalog, conn)?;
        let procedure_template_dag_ty = procedure_template_table.as_ref().dag_ty(conn)?.unwrap();
        let procedure_dag_ty = procedure_table.as_ref().dag_ty(conn)?.unwrap();

        for procedure_template in ProcedureTemplate::load_all(table_catalog, conn)? {
            let procedure_template_name = procedure_template.snake_case_name()?;
            let procedure_template_ident = procedure_template.snake_case_ident()?;
            let procedure_template_type = procedure_template.import_struct_path()?;
            let submodule: std::path::PathBuf =
                root.join(procedure_template_name).with_extension("rs");
            let procedure = procedure_template.procedure(conn)?;
            let procedure_type = procedure.import_struct_path()?;
            submodules.push(quote! {
                mod #procedure_template_ident;
            });

            let mut where_constraint = vec![quote! {
                crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
                C: diesel::connection::LoadConnection
            }];
            let root_dispatch = if procedure_template
                .as_ref()
                .has_most_concrete_table_column(false, conn)?
            {
                where_constraint.push(quote! {
                    Self: web_common_traits::database::MostConcreteVariant<
                        C,
                        Variant = #procedure_template_dag_ty
                    >,
                    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<crate::ProcedureTemplate>,
                    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<&'a crate::ProcedureTemplate>>::Output: diesel::query_dsl::LoadQuery<
                        'a,
                        C,
                        crate::ProcedureTemplateAssetModel,
                    >
                });
                quote! {
                    use web_common_traits::database::MostConcreteVariant;
                    self.most_concrete_variant(conn)?.procedure_template_asset_models(conn)
                }
            } else {
                let mut read_dispatch_ids = Vec::new();
                for (procedure_template_asset_model, _) in
                    procedure_template.procedure_template_asset_models(conn)?
                {
                    let column_ident = procedure_template_asset_model.snake_case_ident()?;
                    read_dispatch_ids.push(column_ident);
                }
                quote! {
                    Ok(vec![
                        #(<Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                            self.#read_dispatch_ids,
                            conn
                        )?),*
                    ])
                }
            };

            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl<C> web_common_traits::prelude::ProcedureTemplate<C> for #procedure_template_type
                    where #(#where_constraint),* {
                        type Procedure = #procedure_type;
                        type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;

                        fn procedure_template_asset_models(
                            &self,
                            conn: &mut C,
                        ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
                            #root_dispatch
                        }
                    }
                })?,
            )?;
        }

        let mut dag_variants =
            self.extension_network.descendants(procedure_template_table.as_ref());
        dag_variants.sort_unstable();

        let procedure_template_idents =
            dag_variants.iter().map(|table| table.struct_ident()).collect::<Result<Vec<_>, _>>()?;

        let submodule_path = root.with_extension("rs");
        std::fs::write(
            submodule_path,
            self.beautify_code(&quote! {
                #(#submodules)*

                impl<C> web_common_traits::prelude::ProcedureTemplate<C> for #procedure_template_dag_ty
                    where
                    C: diesel::connection::LoadConnection,
                    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
                    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<crate::ProcedureTemplate>,
                    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<&'a crate::ProcedureTemplate>>::Output: diesel::query_dsl::LoadQuery<
                        'a,
                        C,
                        crate::ProcedureTemplateAssetModel,
                    >,
                {
                    type Procedure = #procedure_dag_ty;
                    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;

                    fn procedure_template_asset_models(&self, conn: &mut C) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
                        Ok(match self {
                            #(Self::#procedure_template_idents(value) => value.procedure_template_asset_models(conn)?,)*
                            Self::#procedure_template_ident(value) => {
                                use diesel::{BelongingToDsl, RunQueryDsl};
                                Self::ProcedureTemplateAssetModel::belonging_to(value).load(conn)?
                            }
                        })
                    }
                }
            })?,
        )?;

        Ok(())
    }
}
