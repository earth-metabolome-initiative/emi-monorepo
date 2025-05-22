#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepToolModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepToolModelBuilder;
}
