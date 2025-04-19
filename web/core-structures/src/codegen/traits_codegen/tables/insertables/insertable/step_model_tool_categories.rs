#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelToolCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelToolCategoryBuilder;
}
