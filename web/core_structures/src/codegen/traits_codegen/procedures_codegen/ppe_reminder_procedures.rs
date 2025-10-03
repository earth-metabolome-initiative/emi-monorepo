impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure
{
    type Template = crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![(self.procedure_template_ppe_asset_model, self.procedure_ppe_asset)]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::PpeReminderProcedureSettable;
        if let Some(procedure_ppe_asset) =
            template_graph.procedure_asset(parents, template.procedure_template_ppe_asset_model)
        {
            self = self.procedure_ppe_asset(procedure_ppe_asset)?;
        }
        Ok(self)
    }
}
