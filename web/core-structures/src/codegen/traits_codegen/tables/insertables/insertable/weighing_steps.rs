#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepBuilder;
}
