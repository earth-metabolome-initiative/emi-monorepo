impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(vec![])
    }
}
