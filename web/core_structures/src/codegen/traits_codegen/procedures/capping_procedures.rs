impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate;
}
