#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepModel;
}
