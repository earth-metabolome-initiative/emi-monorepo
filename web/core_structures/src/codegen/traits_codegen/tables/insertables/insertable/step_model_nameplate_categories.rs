#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelNameplateCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelNameplateCategoryBuilder;
}
