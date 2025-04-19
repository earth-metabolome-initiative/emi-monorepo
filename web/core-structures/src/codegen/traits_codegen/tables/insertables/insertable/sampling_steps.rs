#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepBuilder;
}
