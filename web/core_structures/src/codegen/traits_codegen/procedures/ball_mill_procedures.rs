impl<C> web_common_traits::prelude::Procedure<C>
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate;
}
