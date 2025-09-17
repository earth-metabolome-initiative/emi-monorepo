impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure
{
    type Template = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder;
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
                self.procedure_template_centrifuged_container_model,
                self.procedure_centrifuged_container,
            ),
            (self.procedure_template_centrifuged_with_model, self.procedure_centrifuged_with),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable;
        if let Some(procedure_centrifuged_container) = template_graph
            .procedure_asset(parents, template.procedure_template_centrifuged_container_model)
        {
            self = self.procedure_centrifuged_container(procedure_centrifuged_container)?;
        }
        if let Some(procedure_centrifuged_with) = template_graph
            .procedure_asset(parents, template.procedure_template_centrifuged_with_model)
        {
            self = self.procedure_centrifuged_with(procedure_centrifuged_with)?;
        }
        Ok(self)
    }
}
