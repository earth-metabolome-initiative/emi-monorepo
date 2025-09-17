impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure
{
    type Template = crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (
                self.procedure_template_freeze_dried_container_model,
                self.procedure_freeze_dried_container,
            ),
            (self.procedure_template_freeze_dried_with_model, self.procedure_freeze_dried_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable;
        if let Some(procedure_freeze_dried_container) = template_graph
            .procedure_asset(parents, template.procedure_template_freeze_dried_container_model)
        {
            self = self.procedure_freeze_dried_container(procedure_freeze_dried_container)?;
        }
        if let Some(procedure_freeze_dried_with) = template_graph
            .procedure_asset(parents, template.procedure_template_freeze_dried_with_model)
        {
            self = self.procedure_freeze_dried_with(procedure_freeze_dried_with)?;
        }
        Ok(self)
    }
}
