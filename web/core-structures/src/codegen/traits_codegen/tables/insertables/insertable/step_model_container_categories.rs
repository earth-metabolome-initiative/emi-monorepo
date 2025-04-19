#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelContainerCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelContainerCategoryBuilder;
}
