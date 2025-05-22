#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSamplingStepModelBuilder;
}
