impl<C> web_common_traits::prelude::Procedure<C> for crate::Procedure
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
    crate::ProcedureTemplate: web_common_traits::database::MostConcreteVariant<
        C,
        Variant = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG,
    >,
    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<
        crate::ProcedureTemplate,
    >,
    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<
        &'a crate::ProcedureTemplate,
    >>::Output: diesel::query_dsl::LoadQuery<'a, C, crate::ProcedureTemplateAssetModel>,
{
    type Template = crate::ProcedureTemplate;
}
