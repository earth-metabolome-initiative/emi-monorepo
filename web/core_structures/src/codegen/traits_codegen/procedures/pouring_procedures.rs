impl<C> web_common_traits::prelude::Procedure<C> for crate::PouringProcedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
{
    type Template = crate::PouringProcedureTemplate;
}
