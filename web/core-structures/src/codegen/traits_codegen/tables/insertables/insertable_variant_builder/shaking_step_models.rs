#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableShakingStepModel;
}
