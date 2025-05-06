#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepModelBuilder;
}
