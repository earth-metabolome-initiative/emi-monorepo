#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableShakingStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepBuilder;
}
