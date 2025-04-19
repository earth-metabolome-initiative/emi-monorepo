#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelCategoryBuilder;
}
