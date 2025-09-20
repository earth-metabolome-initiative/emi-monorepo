impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure
{
    type Template = crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![(self.procedure_template_disposed_asset_model, self.procedure_disposed_asset)]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable;
        if let Some(procedure_disposed_asset) = template_graph
            .procedure_asset(parents, template.procedure_template_disposed_asset_model)
        {
            self = self.procedure_disposed_asset(procedure_disposed_asset)?;
        }
        Ok(self)
    }
}
