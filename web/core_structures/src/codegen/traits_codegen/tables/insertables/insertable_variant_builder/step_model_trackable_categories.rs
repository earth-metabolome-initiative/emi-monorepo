#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelTrackableCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelTrackableCategory;
}
