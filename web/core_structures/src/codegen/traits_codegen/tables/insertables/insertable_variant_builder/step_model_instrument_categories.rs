#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentCategory;
}
