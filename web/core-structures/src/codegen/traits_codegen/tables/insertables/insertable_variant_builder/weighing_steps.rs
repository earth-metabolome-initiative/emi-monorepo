#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStep;
}
