impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate;
}
