#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepModel;
}
