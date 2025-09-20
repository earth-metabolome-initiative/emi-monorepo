impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure
{
    type Template = crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_photographed_asset_model, self.procedure_photographed_asset),
            (self.procedure_template_photographed_with_model, self.procedure_photographed_with),
            (self.procedure_template_photograph_model, self.procedure_photograph),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable;
        if let Some(procedure_photographed_asset) = template_graph
            .procedure_asset(parents, template.procedure_template_photographed_asset_model)
        {
            self = self.procedure_photographed_asset(procedure_photographed_asset)?;
        }
        if let Some(procedure_photographed_with) = template_graph
            .procedure_asset(parents, template.procedure_template_photographed_with_model)
        {
            self = self.procedure_photographed_with(procedure_photographed_with)?;
        }
        if let Some(procedure_photograph) =
            template_graph.procedure_asset(parents, template.procedure_template_photograph_model)
        {
            self = self.procedure_photograph(procedure_photograph)?;
        }
        Ok(self)
    }
}
