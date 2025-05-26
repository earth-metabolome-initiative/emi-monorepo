#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelTrackableCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelTrackableCategoryBuilder;
}
