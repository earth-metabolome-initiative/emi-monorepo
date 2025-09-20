impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure
{
    type Template = crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        vec![
            (self.procedure_template_poured_from_model, self.procedure_poured_from),
            (self.procedure_template_measured_with_model, self.procedure_measured_with),
            (self.procedure_template_poured_into_model, self.procedure_poured_into),
        ]
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder
{
    type Procedure = crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure;
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
        use crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable;
        if let Some(procedure_poured_from) =
            template_graph.procedure_asset(parents, template.procedure_template_poured_from_model)
        {
            self = self.procedure_poured_from(procedure_poured_from)?;
        }
        if let Some(procedure_measured_with) =
            template_graph.procedure_asset(parents, template.procedure_template_measured_with_model)
        {
            self = self.procedure_measured_with(procedure_measured_with)?;
        }
        if let Some(procedure_poured_into) =
            template_graph.procedure_asset(parents, template.procedure_template_poured_into_model)
        {
            self = self.procedure_poured_into(procedure_poured_into)?;
        }
        Ok(self)
    }
}
