#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelToolCategoryBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelToolCategory;
}
