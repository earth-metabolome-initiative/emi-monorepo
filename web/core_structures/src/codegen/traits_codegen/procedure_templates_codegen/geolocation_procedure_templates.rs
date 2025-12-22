impl web_common_traits::prelude::ProcedureTemplateLike
for crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate {
    type Procedure = crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate
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
                < C >> ::read(self.procedure_template_geolocated_with_model, conn) ?, <
                Self::ProcedureTemplateAssetModel as web_common_traits::database::Read <
                C >> ::read(self.procedure_template_geolocated_asset_model_id, conn) ?
            ],
        )
    }
}
