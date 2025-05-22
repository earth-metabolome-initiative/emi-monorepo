#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentCategoryBuilder;
}
