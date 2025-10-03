impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure
{
    type Template = crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_cleaned_with_model, self.procedure_cleaned_with),
            (self.procedure_template_cleaned_model, self.procedure_cleaned),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable;
        if let Some(procedure_cleaned_with) =
            template_graph.procedure_asset(parents, template.procedure_template_cleaned_with_model)
        {
            self = self.procedure_cleaned_with(procedure_cleaned_with)?;
        }
        if let Some(procedure_cleaned) =
            template_graph.procedure_asset(parents, template.procedure_template_cleaned_model)
        {
            self = self.procedure_cleaned(procedure_cleaned)?;
        }
        Ok(self)
    }
}
