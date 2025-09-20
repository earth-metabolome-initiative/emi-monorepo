impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
{
    type Template = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_bead_model, self.procedure_bead),
            (self.procedure_template_milled_with_model, self.procedure_milled_with),
            (self.procedure_template_milled_container_model, self.procedure_milled_container),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable;
        if let Some(procedure_bead) =
            template_graph.procedure_asset(parents, template.procedure_template_bead_model)
        {
            self = self.procedure_bead(procedure_bead)?;
        }
        if let Some(procedure_milled_with) =
            template_graph.procedure_asset(parents, template.procedure_template_milled_with_model)
        {
            self = self.procedure_milled_with(procedure_milled_with)?;
        }
        if let Some(procedure_milled_container) = template_graph
            .procedure_asset(parents, template.procedure_template_milled_container_model)
        {
            self = self.procedure_milled_container(procedure_milled_container)?;
        }
        Ok(self)
    }
}
