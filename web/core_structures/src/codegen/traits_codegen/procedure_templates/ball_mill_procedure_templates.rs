impl<C> web_common_traits::prelude::ProcedureTemplate<C> for crate::BallMillProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Procedure = crate::BallMillProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(vec![
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_bead_model,
                conn,
            )?,
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_milled_with_model,
                conn,
            )?,
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::Read<C>>::read(
                self.procedure_template_milled_container_model,
                conn,
            )?,
        ])
    }
}
