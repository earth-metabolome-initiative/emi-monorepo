mod aliquoting_procedures;
mod ball_mill_procedures;
mod capping_procedures;
mod centrifuge_procedures;
mod disposal_procedures;
mod fractioning_procedures;
mod freeze_drying_procedures;
mod freezing_procedures;
mod geolocation_procedures;
mod harvesting_procedures;
mod packaging_procedures;
mod photograph_procedures;
mod pouring_procedures;
mod procedures;
mod storage_procedures;
mod supernatant_procedures;
mod weighing_procedures;
impl<C> web_common_traits::prelude::Procedure<C>
for crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureDAG
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<
        crate::ProcedureTemplate,
    >,
    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<
        &'a crate::ProcedureTemplate,
    >>::Output: diesel::query_dsl::LoadQuery<'a, C, crate::ProcedureTemplateAssetModel>,
{
    type Template = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG;
}
