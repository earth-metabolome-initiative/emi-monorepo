impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_frozen_container_model, self.procedure_frozen_container),
            (self.procedure_template_frozen_with_model, self.procedure_frozen_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable;
        if let Some(procedure_frozen_container) = template_graph
            .procedure_asset(parents, template.procedure_template_frozen_container_model)
        {
            self = self.procedure_frozen_container(procedure_frozen_container)?;
        }
        if let Some(procedure_frozen_with) =
            template_graph.procedure_asset(parents, template.procedure_template_frozen_with_model)
        {
            self = self.procedure_frozen_with(procedure_frozen_with)?;
        }
        Ok(self)
    }
}
