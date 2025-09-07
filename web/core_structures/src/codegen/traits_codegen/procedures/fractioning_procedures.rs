impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate;
}
