impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate
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
                < C >> ::read(self.procedure_template_stratified_source_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_supernatant_destination_model, conn)
                ?, < Self::ProcedureTemplateAssetModel as
                web_common_traits::database::Read < C >> ::read(self
                .procedure_template_transferred_with_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_pipette_tip_model, conn) ?
            ],
        )
    }
}
