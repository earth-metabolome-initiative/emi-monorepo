#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingStepBuilder;
}
