#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStep;
}
