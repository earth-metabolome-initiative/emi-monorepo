#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepModelBuilder;
}
