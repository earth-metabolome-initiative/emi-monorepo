#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepBuilder;
}
