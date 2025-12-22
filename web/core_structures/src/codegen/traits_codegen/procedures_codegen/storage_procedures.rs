impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure
{
    type Template = crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_stored_asset_model_id, self.procedure_stored_asset),
            (self.procedure_template_stored_into_model, self.procedure_stored_into),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder
{
    type Procedure = crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable;
        if let Some(procedure_stored_asset) = template_graph
            .procedure_asset(parents, template.procedure_template_stored_asset_model_id)
        {
            self = self.procedure_stored_asset(procedure_stored_asset)?;
        }
        if let Some(procedure_stored_into) =
            template_graph.procedure_asset(parents, template.procedure_template_stored_into_model)
        {
            self = self.procedure_stored_into(procedure_stored_into)?;
        }
        Ok(self)
    }
}
