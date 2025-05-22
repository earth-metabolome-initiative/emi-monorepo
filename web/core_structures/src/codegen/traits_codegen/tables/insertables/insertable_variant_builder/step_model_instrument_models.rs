#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentModelBuilder {
    type Row = crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentModel;
}
