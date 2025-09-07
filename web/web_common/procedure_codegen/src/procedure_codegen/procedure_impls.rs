//! Submodule implementing the generation of the `Procedure` trait for
//! all procedures.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::{Procedure, ProcedureTemplate};

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `Procedure` trait for all
    /// procedures.
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
    pub(super) fn procedure_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        let mut submodules = Vec::new();
        for procedure in Procedure::load_all(table_catalog, conn)? {
            let procedure_name = procedure.snake_case_name()?;
            let procedure_ident = procedure.snake_case_ident()?;
            let procedure_type = procedure.import_struct_path()?;
            let submodule = root.join(procedure_name).with_extension("rs");
            let procedure_template = procedure.procedure_template(conn)?;
            let procedure_template_type = procedure_template.import_struct_path()?;
            submodules.push(quote! {
                mod #procedure_ident;
            });

            let mut where_requirements = vec![quote! {
                crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
                C: diesel::connection::LoadConnection
            }];

            if procedure.is_abstract() {
                where_requirements.push(quote! {
                    crate::ProcedureTemplate: web_common_traits::database::MostConcreteVariant<
                        C,
                        Variant = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG,
                    >,
                    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<crate::ProcedureTemplate>,
                    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<&'a crate::ProcedureTemplate>>::Output: diesel::query_dsl::LoadQuery<
                        'a,
                        C,
                        crate::ProcedureTemplateAssetModel,
                    >
                });
            }

            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl<C> web_common_traits::prelude::Procedure<C> for #procedure_type
                    where #(#where_requirements),*{
                        type Template = #procedure_template_type;
                    }
                })?,
            )?;
        }

        let procedure_table = Procedure::root(table_catalog, conn)?;
        let procedure_dag_ident = procedure_table.as_ref().dag_ty(conn)?.unwrap();
        let procedure_template_table = ProcedureTemplate::root(table_catalog, conn)?;
        let procedure_template_dag_ident = procedure_template_table.as_ref().dag_ty(conn)?.unwrap();

        let submodule_path = root.with_extension("rs");
        std::fs::write(
            submodule_path,
            self.beautify_code(&quote! {
                #(#submodules)*

                impl<C> web_common_traits::prelude::Procedure<C> for #procedure_dag_ident
                where
                    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
                    C: diesel::connection::LoadConnection,
                    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<crate::ProcedureTemplate>,
                    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<&'a crate::ProcedureTemplate>>::Output: diesel::query_dsl::LoadQuery<
                        'a,
                        C,
                        crate::ProcedureTemplateAssetModel,
                    >,
                {
                    type Template = #procedure_template_dag_ident;
                }
            })?,
        )?;

        Ok(())
    }
}
