impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_sample_source_model, self.procedure_sample_source),
            (self.procedure_template_sample_model, self.procedure_sample),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable;
        if let Some(procedure_sample_source) =
            template_graph.procedure_asset(parents, template.procedure_template_sample_source_model)
        {
            self = self.procedure_sample_source(procedure_sample_source)?;
        }
        if let Some(procedure_sample) =
            template_graph.procedure_asset(parents, template.procedure_template_sample_model)
        {
            self = self.procedure_sample(procedure_sample)?;
        }
        Ok(self)
    }
}
