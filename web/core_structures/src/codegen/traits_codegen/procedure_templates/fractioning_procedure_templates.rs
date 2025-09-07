impl<C> web_common_traits::prelude::ProcedureTemplate<C>
for crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Procedure = crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(
            vec![
                < Self::ProcedureTemplateAssetModel as web_common_traits::database::Read
                < C >> ::read(self.procedure_template_weighed_with_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_fragment_container_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_fragment_placed_into_model, conn) ?
            ],
        )
    }
}
