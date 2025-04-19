#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_models::StepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelBuilder;
}
