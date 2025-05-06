#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillStep;
}
