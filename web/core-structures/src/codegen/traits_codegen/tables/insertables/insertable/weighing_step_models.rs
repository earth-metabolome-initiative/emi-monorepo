#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingStepModelBuilder;
}
