#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableGrindingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableGrindingStepModel;
}
