#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_instruments::StepInstrument
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepInstrument;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepInstrumentBuilder;
}
