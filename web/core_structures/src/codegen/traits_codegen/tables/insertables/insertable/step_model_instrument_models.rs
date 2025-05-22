#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentModelBuilder;
}
