impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure
{
    type Template = crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_geolocated_asset_model, self.procedure_geolocated_asset),
            (self.procedure_template_geolocated_with_model, self.procedure_geolocated_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure;
    fn complete_with<G, PT>(
        mut self,
        parents: &[&PT],
        template: &<Self::Procedure as web_common_traits::prelude::ProcedureLike>::Template,
        template_graph: &G,
    ) -> Result<Self, Self::Error>
    where
        G: web_common_traits::prelude::ProcedureTemplateAssetGraph<
            ProcedureTemplateAssetModel = <Self::Procedure as web_common_traits::prelude::ProcedureLike>::ProcedureTemplateAssetModel,
            ProcedureAsset = <Self::Procedure as web_common_traits::prelude::ProcedureLike>::ProcedureAsset,
            ProcedureTemplateRoot = PT,
        >,
    {
        use crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable;
        if let Some(procedure_geolocated_asset) = template_graph
            .procedure_asset(parents, template.procedure_template_geolocated_asset_model)
        {
            self = self.procedure_geolocated_asset(procedure_geolocated_asset)?;
        }
        if let Some(procedure_geolocated_with) = template_graph
            .procedure_asset(parents, template.procedure_template_geolocated_with_model)
        {
            self = self.procedure_geolocated_with(procedure_geolocated_with)?;
        }
        Ok(self)
    }
}
