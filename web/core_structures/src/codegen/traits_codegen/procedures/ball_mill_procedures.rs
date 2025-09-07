impl<C> web_common_traits::prelude::Procedure<C> for crate::BallMillProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::BallMillProcedureTemplate;
}
