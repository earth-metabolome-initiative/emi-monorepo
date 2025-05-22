#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepModel;
}
