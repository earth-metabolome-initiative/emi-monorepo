#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableGrindingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableGrindingStepModelBuilder;
}
