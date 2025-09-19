//! Submodule implementing the generation of the `Procedure` trait for
//! all procedures.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use syn::Ident;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::{Procedure, ProcedureTemplate};

impl ProcedureCodegen<'_> {
    /// Generates the implementation of the `Procedure` trait for all
    /// procedures.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path where to output the generated code.
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a `PostgreSQL` connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        let mut submodules = Vec::new();
        let procedures = Procedure::load_all(table_catalog, conn)?;
        for procedure in &procedures {
            let procedure_name = procedure.snake_case_name()?;
            let procedure_ident = procedure.snake_case_ident()?;
            let procedure_type = procedure.import_struct_path()?;
            let procedure_builder_type = procedure.as_ref().insertable_builder_ty()?;
            let submodule = root.join(procedure_name).with_extension("rs");
            let procedure_template = procedure.procedure_template(conn)?;
            let procedure_template_type = procedure_template.import_struct_path()?;
            submodules.push(quote! {
                mod #procedure_ident;
            });

            let mut procedure_template_asset_models_and_procedure_assets = Vec::new();
            let mut complete_with_dispatches = Vec::new();
            for (procedure_asset, procedure_template_asset_model) in
                procedure.coupled_pa_and_ptams(conn)?
            {
                let procedure_template_asset_model_ident =
                    procedure_template_asset_model.snake_case_ident()?;
                let procedure_asset_ident = procedure_asset.snake_case_ident()?;
                let procedure_asset_setter = procedure_asset.getter_ident()?;
                procedure_template_asset_models_and_procedure_assets.push(quote! {
                    (
                        self.#procedure_template_asset_model_ident,
                        self.#procedure_asset_ident,
                    )
                });
                complete_with_dispatches.push(quote! {
                    if let Some(#procedure_asset_ident) = template_graph.procedure_asset(parents, template.#procedure_template_asset_model_ident) {
                        self = self.#procedure_asset_setter(#procedure_asset_ident)?;
                    }
                });
            }

            let procedure_template_asset_models_and_procedure_assets =
                if procedure_template_asset_models_and_procedure_assets.is_empty() {
                    quote! { Vec::new() }
                } else {
                    quote! { vec![#(#procedure_template_asset_models_and_procedure_assets),*] }
                };

            let mut requirements = Vec::new();

            let (parents_ident, template_ident, template_graph_ident) =
                if complete_with_dispatches.is_empty() {
                    (
                        Ident::new("_parents", proc_macro2::Span::call_site()),
                        Ident::new("_template", proc_macro2::Span::call_site()),
                        Ident::new("_template_graph", proc_macro2::Span::call_site()),
                    )
                } else {
                    requirements.push(procedure.as_ref().setter_trait_ty()?);
                    (
                        Ident::new("parents", proc_macro2::Span::call_site()),
                        Ident::new("template", proc_macro2::Span::call_site()),
                        Ident::new("template_graph", proc_macro2::Span::call_site()),
                    )
                };

            let maybe_mut =
                if complete_with_dispatches.is_empty() { None } else { Some(quote! { mut }) };

            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl web_common_traits::prelude::ProcedureLike for #procedure_type {
                        type Template = #procedure_template_type;
                        type ProcedureAsset = crate::ProcedureAsset;
                        type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
                        type Builder = #procedure_builder_type;

                        fn procedure_template_asset_models_and_procedure_assets(
                            &self,
                        ) -> Vec<(
                            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
                            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
                        )> {
                            #procedure_template_asset_models_and_procedure_assets
                        }
                    }

                    impl web_common_traits::prelude::ProcedureBuilderLike for #procedure_builder_type {
                        type Procedure = #procedure_type;

                        fn complete_with<G, PT>(
                            #maybe_mut self,
                            #parents_ident: &[&PT],
                            #template_ident: &<Self::Procedure as web_common_traits::prelude::ProcedureLike>::Template,
                            #template_graph_ident: &G,
                        ) -> Result<Self, Self::Error> where
                            G: web_common_traits::prelude::ProcedureTemplateAssetGraph<
                                ProcedureTemplateAssetModel = <Self::Procedure as web_common_traits::prelude::ProcedureLike>::ProcedureTemplateAssetModel,
                                ProcedureAsset = <Self::Procedure as web_common_traits::prelude::ProcedureLike>::ProcedureAsset,
                                ProcedureTemplateRoot=PT,
                            >
                        {
                            #(use #requirements;)*
                            #(#complete_with_dispatches)*

                            Ok(self)
                        }
                    }
                })?,
            )?;
        }

        let procedure_table = Procedure::root(table_catalog, conn)?;
        let procedure_template_table = ProcedureTemplate::root(table_catalog, conn)?;
        let procedure_dag_ident = procedure_table.as_ref().dag_ty(conn)?.unwrap();
        let procedure_builder_dag_ident = procedure_table.as_ref().builder_dag_ty(conn)?.unwrap();
        let procedure_template_dag_ident = procedure_template_table.as_ref().dag_ty(conn)?.unwrap();
        let mut variants = Vec::new();
        let mut complete_with_dispatches = Vec::new();

        for procedure in &procedures {
            let procedure_camel_case = procedure.struct_ident()?;
            let procedure_template = procedure.procedure_template(conn)?;
            let procedure_template_camel_case = procedure_template.struct_ident()?;
            complete_with_dispatches.push(quote! {
                (Self::#procedure_camel_case(builder), #procedure_template_dag_ident::#procedure_template_camel_case(template)) => {
                    builder.complete_with(parents, template, template_graph)?.into()
                }
            });
            variants.push(procedure_camel_case);
        }

        let submodule_path = root.with_extension("rs");
        std::fs::write(
            submodule_path,
            self.beautify_code(&quote! {
                #(#submodules)*

                impl web_common_traits::prelude::ProcedureLike for #procedure_dag_ident
                {
                    type Template = #procedure_template_dag_ident;
                    type ProcedureAsset = crate::ProcedureAsset;
                    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
                    type Builder = #procedure_builder_dag_ident;

                    fn procedure_template_asset_models_and_procedure_assets(
                        &self,
                    ) -> Vec<(
                        <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
                        <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
                    )> {
                        match self {
                            #(Self::#variants(procedure) => procedure.procedure_template_asset_models_and_procedure_assets(),)*
                        }
                    }
                }

                impl web_common_traits::prelude::ProcedureBuilderLike for #procedure_builder_dag_ident {
                    type Procedure = #procedure_dag_ident;

                    fn complete_with<G, PT>(
                        self,
                        parents: &[&PT],
                        template: &#procedure_template_dag_ident,
                        template_graph: &G,
                    ) -> Result<Self, Self::Error> where
                        G: web_common_traits::prelude::ProcedureTemplateAssetGraph<
                            ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel,
                            ProcedureAsset = crate::ProcedureAsset,
                            ProcedureTemplateRoot=PT,
                        >
                    {
                        Ok(match (self, template) {
                            #(#complete_with_dispatches),*,
                            _ => unreachable!("Mismatched procedure builder and template types, which implies error in code generation."),
                        })
                    }
                }
            })?,
        )?;

        Ok(())
    }
}
