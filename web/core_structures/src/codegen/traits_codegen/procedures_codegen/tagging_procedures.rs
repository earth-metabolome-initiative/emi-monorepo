impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_tagged_asset_model, self.procedure_tagged_asset),
            (self.procedure_template_tag_asset_model, self.procedure_tag_asset),
            (self.procedure_template_geolocated_with_model, self.procedure_geolocated_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureBuilder
{
    type Procedure = crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::TaggingProcedureSettable;
        if let Some(procedure_tagged_asset) =
            template_graph.procedure_asset(parents, template.procedure_template_tagged_asset_model)
        {
            self = self.procedure_tagged_asset(procedure_tagged_asset)?;
        }
        if let Some(procedure_tag_asset) =
            template_graph.procedure_asset(parents, template.procedure_template_tag_asset_model)
        {
            self = self.procedure_tag_asset(procedure_tag_asset)?;
        }
        if let Some(procedure_geolocated_with) = template_graph
            .procedure_asset(parents, template.procedure_template_geolocated_with_model)
        {
            self = self.procedure_geolocated_with(procedure_geolocated_with)?;
        }
        Ok(self)
    }
}
