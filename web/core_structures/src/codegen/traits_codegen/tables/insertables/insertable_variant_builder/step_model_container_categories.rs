#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelContainerCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelContainerCategory;
}
