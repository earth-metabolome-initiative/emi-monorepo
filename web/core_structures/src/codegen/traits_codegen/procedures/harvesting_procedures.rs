impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate;
}
