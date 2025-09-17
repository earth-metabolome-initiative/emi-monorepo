impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate
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
                < C >> ::read(self.procedure_template_sample_source_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_sample_model, conn) ?
            ],
        )
    }
}
