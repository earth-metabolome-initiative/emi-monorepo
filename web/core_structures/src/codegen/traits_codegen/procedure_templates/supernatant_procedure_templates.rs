impl<C> web_common_traits::prelude::ProcedureTemplate<C> for crate::SupernatantProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Procedure = crate::SupernatantProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(vec![
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_stratified_source_model,
                conn,
            )?,
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_supernatant_destination_model,
                conn,
            )?,
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_transferred_with_model,
                conn,
            )?,
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_pipette_tip_model,
                conn,
            )?,
        ])
    }
}
