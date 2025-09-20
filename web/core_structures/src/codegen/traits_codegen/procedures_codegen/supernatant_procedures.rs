impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
{
    type Template = crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_stratified_source_model, self.procedure_stratified_source),
            (
                self.procedure_template_supernatant_destination_model,
                self.procedure_supernatant_destination,
            ),
            (self.procedure_template_transferred_with_model, self.procedure_transferred_with),
            (self.procedure_template_pipette_tip_model, self.procedure_pipette_tip),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder
{
    type Procedure =
        crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable;
        if let Some(procedure_stratified_source) = template_graph
            .procedure_asset(parents, template.procedure_template_stratified_source_model)
        {
            self = self.procedure_stratified_source(procedure_stratified_source)?;
        }
        if let Some(procedure_supernatant_destination) = template_graph
            .procedure_asset(parents, template.procedure_template_supernatant_destination_model)
        {
            self = self.procedure_supernatant_destination(procedure_supernatant_destination)?;
        }
        if let Some(procedure_transferred_with) = template_graph
            .procedure_asset(parents, template.procedure_template_transferred_with_model)
        {
            self = self.procedure_transferred_with(procedure_transferred_with)?;
        }
        if let Some(procedure_pipette_tip) =
            template_graph.procedure_asset(parents, template.procedure_template_pipette_tip_model)
        {
            self = self.procedure_pipette_tip(procedure_pipette_tip)?;
        }
        Ok(self)
    }
}
