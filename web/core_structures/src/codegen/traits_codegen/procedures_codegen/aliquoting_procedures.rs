impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_aliquoted_with_model, self.procedure_aliquoted_with),
            (self.procedure_template_pipette_tip_model, self.procedure_pipette_tip),
            (self.procedure_template_aliquoted_from_model, self.procedure_aliquoted_from),
            (self.procedure_template_aliquoted_into_model, self.procedure_aliquoted_into),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable;
        if let Some(procedure_aliquoted_with) = template_graph
            .procedure_asset(parents, template.procedure_template_aliquoted_with_model)
        {
            self = self.procedure_aliquoted_with(procedure_aliquoted_with)?;
        }
        if let Some(procedure_pipette_tip) =
            template_graph.procedure_asset(parents, template.procedure_template_pipette_tip_model)
        {
            self = self.procedure_pipette_tip(procedure_pipette_tip)?;
        }
        if let Some(procedure_aliquoted_from) = template_graph
            .procedure_asset(parents, template.procedure_template_aliquoted_from_model)
        {
            self = self.procedure_aliquoted_from(procedure_aliquoted_from)?;
        }
        if let Some(procedure_aliquoted_into) = template_graph
            .procedure_asset(parents, template.procedure_template_aliquoted_into_model)
        {
            self = self.procedure_aliquoted_into(procedure_aliquoted_into)?;
        }
        Ok(self)
    }
}
