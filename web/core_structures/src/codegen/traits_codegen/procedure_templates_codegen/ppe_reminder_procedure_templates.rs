impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate
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
                < C >> ::read(self.procedure_template_ppe_asset_model, conn) ?
            ],
        )
    }
}
