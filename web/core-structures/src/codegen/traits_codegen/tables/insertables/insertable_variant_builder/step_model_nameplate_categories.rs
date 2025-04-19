#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelNameplateCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelNameplateCategory;
}
