impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(
            vec![
                < Self::ProcedureTemplateAssetModel as web_common_traits::database::Read
                < C >> ::read(self.procedure_template_disposed_asset_model_id, conn) ?
            ],
        )
    }
}
