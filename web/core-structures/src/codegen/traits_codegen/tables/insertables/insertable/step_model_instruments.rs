#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrument;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelInstrumentBuilder;
}
