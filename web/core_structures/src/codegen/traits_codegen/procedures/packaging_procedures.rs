impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate;
}
