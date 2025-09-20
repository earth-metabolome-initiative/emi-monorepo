impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_weighed_container_model, self.procedure_weighed_container),
            (self.procedure_template_weighed_with_model, self.procedure_weighed_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable;
        if let Some(procedure_weighed_container) = template_graph
            .procedure_asset(parents, template.procedure_template_weighed_container_model)
        {
            self = self.procedure_weighed_container(procedure_weighed_container)?;
        }
        if let Some(procedure_weighed_with) =
            template_graph.procedure_asset(parents, template.procedure_template_weighed_with_model)
        {
            self = self.procedure_weighed_with(procedure_weighed_with)?;
        }
        Ok(self)
    }
}
